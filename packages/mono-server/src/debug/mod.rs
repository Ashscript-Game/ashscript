//! Server-side debugging and observability helpers.
//!
//! This module is always compiled but runtime-gated via environment variables,
//! so it is effectively a no-op (and cheap) unless explicitly enabled. See
//! [`TickRecorder`] for the replayable NDJSON tick-trace and [`Metrics`] for the
//! rolling run-wide counters.

use std::{
    fs::File,
    io::{BufWriter, Write},
    time::Duration,
};

use ashscript_types::actions::{ActionCounts, ActionsByKind};
use serde::Serialize;
use tracing::{info, warn};

/// Environment variable naming the file that receives the NDJSON tick trace.
const TRACE_FILE_ENV: &str = "ASHSCRIPT_TRACE_FILE";

/// Run-wide counters accumulated across ticks.
///
/// Kept intentionally modest: a tick count, a grand total of actions, and a
/// per-kind running total derived from each tick's [`ActionCounts`].
#[derive(Debug, Default, Serialize)]
pub struct Metrics {
    pub ticks: u64,
    pub total_actions: u64,
    pub unit_move: u64,
    pub unit_attack: u64,
    pub turret_attack: u64,
    pub factory_spawn_unit: u64,
    pub unit_spawn_unit: u64,
    pub resource_transfer: u64,
    pub turret_repair: u64,
    pub substation_collect: u64,
    pub extract_resource: u64,
}

impl Metrics {
    /// Fold a single tick's action counts into the running totals.
    pub fn accumulate(&mut self, counts: &ActionCounts) {
        self.ticks += 1;
        self.total_actions += counts.total() as u64;
        self.unit_move += counts.unit_move as u64;
        self.unit_attack += counts.unit_attack as u64;
        self.turret_attack += counts.turret_attack as u64;
        self.factory_spawn_unit += counts.factory_spawn_unit as u64;
        self.unit_spawn_unit += counts.unit_spawn_unit as u64;
        self.resource_transfer += counts.resource_transfer as u64;
        self.turret_repair += counts.turret_repair as u64;
        self.substation_collect += counts.substation_collect as u64;
        self.extract_resource += counts.extract_resource as u64;
    }

    /// Emit a one-line rolling summary at info level every `interval` ticks.
    pub fn maybe_log_summary(&self, interval: u64) {
        if interval != 0 && self.ticks % interval == 0 {
            info!(
                ticks = self.ticks,
                total_actions = self.total_actions,
                "metrics summary"
            );
        }
    }
}

/// One line of the NDJSON tick trace.
#[derive(Serialize)]
struct TickRecord<'a> {
    tick: u64,
    dur_ms: f64,
    counts: ActionCounts,
    actions: &'a ActionsByKind,
    metrics: &'a Metrics,
}

/// Append-only NDJSON recorder of per-tick state for offline replay/analysis.
///
/// Enabled only when [`TRACE_FILE_ENV`] (`ASHSCRIPT_TRACE_FILE`) is set;
/// otherwise it is a no-op and `record` returns immediately. Always compiled.
#[derive(Default)]
pub struct TickRecorder {
    writer: Option<BufWriter<File>>,
}

impl TickRecorder {
    /// Build a recorder from the environment.
    ///
    /// When `ASHSCRIPT_TRACE_FILE` is set to a non-empty path, the file is
    /// opened in append/create mode. Any failure to open it is logged and
    /// degrades gracefully to a disabled, no-op recorder.
    pub fn from_env() -> Self {
        let Some(path) = std::env::var(TRACE_FILE_ENV).ok().filter(|p| !p.is_empty()) else {
            return Self::default();
        };

        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            Ok(file) => {
                info!(path = %path, "tick trace recording enabled");
                Self {
                    writer: Some(BufWriter::new(file)),
                }
            }
            Err(err) => {
                warn!(path = %path, %err, "failed to open tick trace file; recording disabled");
                Self::default()
            }
        }
    }

    /// Returns whether trace recording is active.
    pub fn is_enabled(&self) -> bool {
        self.writer.is_some()
    }

    /// Append one NDJSON line describing this tick, then flush.
    ///
    /// A no-op when recording is disabled. Serialization/IO errors are logged
    /// rather than propagated so a bad trace file never disrupts the sim.
    pub fn record(
        &mut self,
        tick: u64,
        dur: Duration,
        actions: &ActionsByKind,
        metrics: &Metrics,
    ) {
        let Some(writer) = self.writer.as_mut() else {
            return;
        };

        let record = TickRecord {
            tick,
            dur_ms: dur.as_secs_f64() * 1_000.0,
            counts: actions.counts(),
            actions,
            metrics,
        };

        match serde_json::to_writer(&mut *writer, &record)
            .map_err(std::io::Error::from)
            .and_then(|()| writer.write_all(b"\n"))
            .and_then(|()| writer.flush())
        {
            Ok(()) => {}
            Err(err) => warn!(%err, "failed to write tick trace record"),
        }
    }
}
