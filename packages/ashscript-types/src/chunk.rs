use std::hash::Hash;

use enum_map::EnumMap;
use hashbrown::HashMap;
use hecs::Entity;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::objects::GameObjectKind;

pub type Chunks = HashMap<Hex, Chunk>;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
    pub hex: Hex,
    pub entities: EnumMap<GameObjectKind, HashMap<Hex, Entity>>,
    pub solar_efficiency: f32,
    pub wind_efficiency: f32,
}

impl Chunk {
    pub fn new(hex: Hex, noise: f64) -> Self {
        Self {
            hex,
            solar_efficiency: noise as f32,
            wind_efficiency: noise as f32,
            ..Default::default()
        }
    }
}
