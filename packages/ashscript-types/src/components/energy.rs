use serde::{Deserialize, Serialize};

use crate::{constants::structures::GAME_OBJECT_ENERGY_CAPACITIES, objects::GameObjectKind};

use super::body::UnitBody;

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Energy {
    pub current: u32,
    pub capacity: u32,
}

impl Energy {
    pub fn for_unit_body(unit_body: &UnitBody) -> Self {
        Self {
            capacity: unit_body.energy_capacity(),
            ..Default::default()
        }
    }

    pub fn for_structure(kind: &GameObjectKind) -> Self {

        let Some(capacity) = GAME_OBJECT_ENERGY_CAPACITIES.get(kind) else {
            return Default::default();
        };

        Self {
            capacity: *capacity,
            ..Default::default()
        }
    }
}