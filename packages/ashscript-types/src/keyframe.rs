use hecs::World;
use serde::{Deserialize, Serialize};
use postcard::to_stdvec;
use bitcode::serialize;

use crate::{actions::{self, ActionsByKind}, global::Global, map::Map, world::serialize_world_data};

#[derive(Default, Serialize, Deserialize)]
pub struct KeyFrame {
    pub map: Map,
    // pub world: World,
    pub world_data: Vec<u8>,
    pub global: Global,
    pub actions: ActionsByKind,
}

impl KeyFrame {
    pub fn from_existing(map: Map, world: &World, global: Global, actions: ActionsByKind) -> Self {
        Self {
            map,
            world_data: serialize_world_data(world),
            global,
            actions,
        }
    }
}