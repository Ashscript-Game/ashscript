use ashscript_types::constants::map::{CHUNK_SIZE, HEX_LAYOUT, HEX_SIZE};
use bevy::{color::palettes::css, prelude::*};
use hexx::{shapes, Hex};

use crate::components::{Actions, DebugUI, LoadedChunks};
use crate::constants;

/// Convert a hex coordinate into a 2D world position using the shared
/// `HEX_LAYOUT` so overlays line up exactly with rendered tiles/entities.
fn hex_to_world(hex: Hex) -> Vec2 {
    HEX_LAYOUT.hex_to_world_pos(hex)
}

/// Draws an arrowhead at `to`, pointing along the `from -> to` direction.
fn draw_arrow(gizmos: &mut Gizmos, from: Vec2, to: Vec2, color: Srgba) {
    gizmos.line_2d(from, to, color);

    let dir = (to - from).normalize_or_zero();
    if dir == Vec2::ZERO {
        return;
    }

    // Two short barbs splayed back from the tip.
    let head_len = HEX_SIZE.x * 0.35;
    let left = Vec2::new(-dir.y, dir.x);
    let base = to - dir * head_len;
    gizmos.line_2d(to, base + left * head_len * 0.5, color);
    gizmos.line_2d(to, base - left * head_len * 0.5, color);
}

/// Renders all enabled debug gizmo overlays. Each section is fully skipped when
/// its toggle is off so overlays stay cheap.
pub fn draw_debug_gizmos(
    mut gizmos: Gizmos,
    debug_ui: Res<DebugUI>,
    actions: Res<Actions>,
    loaded_chunks: Res<LoadedChunks>,
) {
    if !debug_ui.enabled {
        return;
    }

    // Chunk / hex grid: outline every hex of every loaded chunk.
    if debug_ui.chunk_lines {
        let grid_color = css::DARK_GRAY.with_alpha(0.5);
        for chunk_hex in loaded_chunks.0.iter() {
            for hex in shapes::hexagon(chunk_hex.to_higher_res(CHUNK_SIZE), CHUNK_SIZE) {
                let corners = HEX_LAYOUT.hex_corners(hex);
                for i in 0..6 {
                    gizmos.line_2d(corners[i], corners[(i + 1) % 6], grid_color);
                }
            }
        }
    }

    // Hex coordinate markers: a small axis cross at each loaded hex center,
    // marking the lattice points (gizmos can't render text). The +x leg points
    // along the hex `q` axis and the +y leg along world up, so the orientation
    // of the hex grid is readable at a glance.
    if debug_ui.hex_coords {
        let marker_color = css::FUCHSIA.with_alpha(0.7);
        let arm = HEX_SIZE.x * 0.15;
        for chunk_hex in loaded_chunks.0.iter() {
            for hex in shapes::hexagon(chunk_hex.to_higher_res(CHUNK_SIZE), CHUNK_SIZE) {
                let center = hex_to_world(hex);
                gizmos.line_2d(center - Vec2::X * arm, center + Vec2::X * arm, marker_color);
                gizmos.line_2d(center - Vec2::Y * arm, center + Vec2::Y * arm, marker_color);
            }
        }
    }

    // Action arrows: visualize movement and attacks from the latest keyframe.
    if debug_ui.action_arrows {
        for action in actions.0.unit_move.iter() {
            draw_arrow(
                &mut gizmos,
                hex_to_world(action.from),
                hex_to_world(action.to),
                css::LIME,
            );
        }

        for action in actions.0.unit_attack.iter() {
            draw_arrow(
                &mut gizmos,
                hex_to_world(action.attacker_hex),
                hex_to_world(action.target_hex),
                css::ORANGE_RED,
            );
        }

        for action in actions.0.turret_attack.iter() {
            draw_arrow(
                &mut gizmos,
                hex_to_world(action.turret_hex),
                hex_to_world(action.target_hex),
                css::YELLOW,
            );
        }
    }

    // Range circles: draw a ring at each turret/structure attack origin sized to
    // its configured range (in hexes -> world units).
    if debug_ui.ranges {
        let range_color = css::AQUA.with_alpha(0.4);
        // Turrets have no dedicated range constant; reuse the assembler range as a
        // representative structure radius. Convert hex range -> world units via the
        // shared layout (distance between a hex and one `range` steps away) so the
        // ring matches the actual reach instead of guessing from `HEX_SIZE`.
        let range_hexes = constants::assembler::RANGE as i32;
        let radius = HEX_LAYOUT
            .hex_to_world_pos(Hex::new(range_hexes, 0))
            .distance(HEX_LAYOUT.hex_to_world_pos(Hex::ZERO));
        for action in actions.0.turret_attack.iter() {
            let center = hex_to_world(action.turret_hex);
            gizmos.circle_2d(center, radius, range_color);
        }
    }
}
