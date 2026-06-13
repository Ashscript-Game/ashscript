use glam::Vec2;
use hexx::{HexLayout, HexOrientation};

/// allows for a rectangle with 2 025 000 000 tiles
pub const MAX_WIDTH_HEIGHT: i32 = 45000;

pub const HEX_SIZE: Vec2 = Vec2::splat(64.0);
pub const CHUNK_SIZE: u32 = 5;
pub const HEX_LAYOUT: HexLayout = HexLayout {
    hex_size: HEX_SIZE,
    orientation: HexOrientation::Flat,
    origin: Vec2::new(0., 0.),
    invert_x: false,
    invert_y: false,
};