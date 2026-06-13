use bevy::{
    app::{App, Plugin, Update},
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
    },
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};
use bevy_magic_light_2d::gi::render_layer::ALL_LAYERS;

use crate::components::{
    Actions, DebugUI, FpsText, NetDebugStats, SelectedGameObjects, State,
};
use crate::debug::gizmos::draw_debug_gizmos;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_debug_ui, debug_window, draw_debug_gizmos));
    }
}

fn toggle_debug_ui(mut debug_ui: ResMut<DebugUI>, input: Res<ButtonInput<KeyCode>>) {
    // `just_pressed` so one keypress toggles once instead of flipping every frame held.
    if input.just_pressed(KeyCode::F5) {
        debug_ui.enabled = !debug_ui.enabled;
    }
}

fn debug_window(
    mut egui: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    mut debug_ui: ResMut<DebugUI>,
    state: Res<State>,
    actions: Res<Actions>,
    net_stats: Res<NetDebugStats>,
    selected: Res<SelectedGameObjects>,
) {
    if !debug_ui.enabled {
        return;
    }

    egui::Window::new("Debug")
        .anchor(egui::Align2::RIGHT_TOP, [0., 0.])
        .resizable(true)
        .show(egui.ctx_mut().unwrap(), |ui| {
            egui::CollapsingHeader::new("Perf")
                .default_open(true)
                .show(ui, |ui| {
                    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(value) = fps.smoothed() {
                            ui.label(format!("FPS: {:.1}", value));
                        }
                        if let Some(value) = fps.average() {
                            ui.label(format!("Avg FPS: {:.1}", value));
                        }
                    }
                    if let Some(frame_time) =
                        diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
                    {
                        if let Some(value) = frame_time.smoothed() {
                            ui.label(format!("Frame time: {:.2} ms", value));
                        }
                    }
                    if let Some(count) =
                        diagnostics.get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                    {
                        if let Some(value) = count.value() {
                            ui.label(format!("Entities: {}", value as u64));
                        }
                    }
                });

            egui::CollapsingHeader::new("Sim / Net")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label(format!("Server tick: {}", state.global.tick));
                    ui.label(format!(
                        "Last tick duration: {:.2} ms",
                        state.global.last_tick_duration.as_secs_f64() * 1000.
                    ));
                    ui.label(format!("World entities: {}", state.world.len()));
                    ui.label(format!(
                        "Last keyframe: {} bytes",
                        net_stats.last_keyframe_bytes
                    ));
                    ui.label(format!(
                        "Keyframes/sec: {:.1}",
                        net_stats.keyframes_per_sec
                    ));
                    ui.label(format!("Keyframes total: {}", net_stats.keyframe_count));
                });

            egui::CollapsingHeader::new("Actions")
                .default_open(true)
                .show(ui, |ui| {
                    let counts = actions.0.counts();
                    ui.label(format!("Total: {}", counts.total()));
                    ui.label(counts.to_string());

                    ui.separator();
                    egui::Grid::new("actions_grid")
                        .num_columns(2)
                        .striped(true)
                        .show(ui, |ui| {
                            for (kind, count) in counts.entries() {
                                ui.label(kind);
                                ui.label(count.to_string());
                                ui.end_row();
                            }
                        });
                });

            egui::CollapsingHeader::new("Selection")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label(format!("Selected: {}", selected.0.len()));
                    for entity in selected.0.iter() {
                        ui.label(format!("{entity:?}"));
                    }
                });

            egui::CollapsingHeader::new("Overlays")
                .default_open(true)
                .show(ui, |ui| {
                    ui.checkbox(&mut debug_ui.chunk_lines, "Chunk / hex grid");
                    ui.checkbox(&mut debug_ui.action_arrows, "Action arrows");
                    ui.checkbox(&mut debug_ui.ranges, "Range circles");
                    ui.checkbox(&mut debug_ui.hex_coords, "Hex coordinates");
                });
        });
}
