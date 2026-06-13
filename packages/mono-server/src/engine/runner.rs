use crate::{
    debug::{Metrics, TickRecorder},
    engine::{
        actions::{process_actions::process_actions, server_actions::{self, server_actions}}, client::emit_tick, components::delete_0_health, game_objects::update_resources, unit::units_generate_energy
    },
    game_state::GameState, simulations,
};
use std::{arch::x86_64, sync::Arc, time::{self, Duration}};
use ashscript_types::components::health::Health;
use tokio::{sync::broadcast::Sender, time::sleep};
use tracing::{debug, trace};

use super::{
    actions::create_actions::get_bot_actions,
    unit::{age_units, delete_old_units},
};

/// How often (in ticks) to emit the rolling [`Metrics`] summary line.
const METRICS_SUMMARY_INTERVAL: u64 = 20;

pub async fn runner(game_state: &mut GameState, mut sender: Sender<Arc<Vec<u8>>>) {
    // Observability state lives for the whole run and is threaded into each
    // tick. Both are cheap no-ops unless enabled via their env vars.
    let mut recorder = TickRecorder::from_env();
    let mut metrics = Metrics::default();

    loop {
        tick(game_state, &mut sender, &mut recorder, &mut metrics).await;
    }
}

pub async fn tick(
    game_state: &mut GameState,
    sender: &mut Sender<Arc<Vec<u8>>>,
    recorder: &mut TickRecorder,
    metrics: &mut Metrics,
) {
    let start_time = time::Instant::now();

    // The span guard (`EnteredSpan`) is `!Send` and must not be held across the
    // pacing `.await` below, so all synchronous work for the tick runs inside
    // this scoped block; the guard is dropped before we yield.
    {
        let _span = tracing::info_span!("tick", n = game_state.global.tick).entered();

        let mut actions_by_kind = get_bot_actions(game_state);
        server_actions(game_state, &mut actions_by_kind);

        emit_tick(game_state, &actions_by_kind, sender);

        process_actions(game_state, &actions_by_kind);

        // The compute work for this tick is done; capture its duration before the
        // artificial pacing sleep so logs/traces/the recorder reflect real compute
        // time. Note this is distinct from `last_tick_duration` below, which the
        // client uses as an animation-pacing divisor and must keep its original
        // full-interval meaning.
        let compute = start_time.elapsed();
        let compute_ms = compute.as_secs_f64() * 1_000.0;
        let counts = actions_by_kind.counts();

        debug!(
            tick = game_state.global.tick,
            dur_ms = %compute_ms,
            actions = %counts,
            total = counts.total(),
            "tick complete"
        );
        trace!(?actions_by_kind, "actions detail");

        metrics.accumulate(&counts);
        metrics.maybe_log_summary(METRICS_SUMMARY_INTERVAL);
        recorder.record(game_state.global.tick, compute, &actions_by_kind, metrics);

        age_units(game_state);
        delete_old_units(game_state);
        delete_0_health(game_state);
        units_generate_energy(game_state);

        update_resources(game_state);

        game_state.global.tick += 1;

        simulations::basic::update(game_state);
    }

    sleep(Duration::from_millis(500)).await;

    // Record how long the tick took, including the pacing sleep. The client
    // divides by this to interpolate animation between keyframes, so it must
    // reflect the full wall-clock interval (~the pacing duration), as before.
    game_state.global.last_tick_duration = start_time.elapsed();
}