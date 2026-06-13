// TODO: remove these eventually, once there's less warnings in between the actual
// error messages.
#![allow(unused_imports)]
#![allow(unused_parens)]

use std::net::{SocketAddr, UdpSocket};

use ashscript_types::{actions::ActionsByKind, global::Global, map::Map};
use bevy::{
    app::App,
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    log::{Level, LogPlugin},
    platform::collections::HashMap,
    prelude::*,
    DefaultPlugins,
};
use bevy_magic_light_2d::{gi::BevyMagicLight2DPlugin, prelude::*};
use components::{
    Actions, DebugUI, GameSettings, GameState, LoadedChunks, NetDebugStats, PlayerStates, ProjectileMoveEndTimer, SelectedGameObjects, State, UnloadedChunks
};
use constants::{PROJECTILE_MOVE_END_TICK_PORTION, SECONDS_PER_TICK};
use game::GamePlugin;
use serde::{Deserialize, Serialize};
use bevy_hanabi::prelude::*;

pub mod components;
pub mod constants;
pub mod controls;
pub mod debug;
pub mod engine;
pub mod game;
pub mod lighting;
pub mod networker;
pub mod prelude;
pub mod projectile;
pub mod structure;
pub mod types;
pub mod unit;
pub mod utils;
pub mod ui;
pub mod camera;
pub mod resources;

fn main() {
    let network_info = networker::create_network_resource();

    App::new()
        .insert_resource(ClearColor(Color::srgba(0., 0., 0., 0.)))
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Scripter".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "info,bevy_client=debug,wgpu=error,naga=warn".into(),
                    ..default()
                }),
            GamePlugin,
            BevyMagicLight2DPlugin,
            HanabiPlugin,
            bevy_egui::EguiPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
        ))
        .add_plugins(DebugToolsPlugin)
        .insert_resource(BevyMagicLight2DSettings {
            light_pass_params: LightPassParams {
                reservoir_size: 1, /* 16 */
                smooth_kernel_size: (3, 3),
                direct_light_contrib: 0.2,
                indirect_light_contrib: 0.8,
                ..default()
            },
            ..default()
        })
        .insert_resource(ProjectileMoveEndTimer(Timer::from_seconds(
            SECONDS_PER_TICK * PROJECTILE_MOVE_END_TICK_PORTION,
            TimerMode::Once,
        )))
        .insert_resource(GameSettings { lights: true })
        .insert_resource(State {
            map: Map::new(),
            global: Global::new(),
            world: hecs::World::new(),
        })
        .insert_resource(Actions(ActionsByKind::new()))
        .insert_resource(GameState::new())
        .insert_resource(PlayerStates(HashMap::new()))
        .insert_resource(network_info)
        .register_type::<LightOccluder2D>()
        .register_type::<OmniLightSource2D>()
        .register_type::<SkylightMask2D>()
        .register_type::<SkylightLight2D>()
        .register_type::<BevyMagicLight2DSettings>()
        .register_type::<LightPassParams>()
        .insert_resource(LoadedChunks::default())
        .insert_resource(UnloadedChunks::default())
        .insert_resource(SelectedGameObjects::default())
        .insert_resource(DebugUI::default())
        .insert_resource(NetDebugStats::default())
        .run();
}

/// Bundles dev-only debugging tooling so the `#[cfg]` gating lives in one place
/// instead of being scattered through the main plugin tuple.
struct DebugToolsPlugin;

impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        // `LogDiagnosticsPlugin` is noisy, so it only runs under `--features dev`.
        #[cfg(feature = "dev")]
        app.add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default());

        // Generic egui world inspector, toggled with F3. `bevy-inspector-egui`
        // 0.36 is version-aligned with this client's `bevy_egui` 0.39, so it
        // binds to the same `EguiContext` the rest of the UI uses. It piggybacks
        // on the `EguiPlugin` already added in `main`, so it must come after it
        // (it does — this plugin is added after the main plugin tuple).
        // `input_toggle_active(false, ...)` starts hidden and flips on each F3.
        #[cfg(feature = "dev")]
        app.add_plugins(
            bevy_inspector_egui::quick::WorldInspectorPlugin::new().run_if(
                bevy::input::common_conditions::input_toggle_active(false, KeyCode::F3),
            ),
        );

        // Suppress unused-parameter warning when the `dev` feature is disabled.
        let _ = app;
    }
}
