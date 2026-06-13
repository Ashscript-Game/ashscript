use std::default;

use hexx::Hex;
use serde::{Deserialize, Serialize};

use crate::entity::Component;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Tile {
    pub hex: Hex,
    pub previous_hex: Option<Hex>,
}

impl Tile {
    pub fn new(hex: Hex) -> Self {
        Self {
            hex,
            ..Default::default()
        }
    }
}