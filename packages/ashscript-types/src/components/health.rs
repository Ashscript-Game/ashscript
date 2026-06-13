use serde::{Deserialize, Serialize};

use crate::{constants::structures::GAME_OBJECT_HEALTHS, objects::GameObjectKind};

use super::body::UnitBody;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
/// Excludes shield health
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: 100,
            max: 100,
        }
    }
}

impl Health {
    pub fn from_kind(kind: GameObjectKind) -> Self {
        let Some(health) = GAME_OBJECT_HEALTHS.get(&kind) else {
            return Default::default();
        };
        Self {
            current: *health,
            max: *health,
        }
    }

    pub fn for_unit(body: &UnitBody) -> Self {
        Self {
            current: body.max_health(),
            max: body.max_health(),
        }
    }

    pub fn current_with_shields() {
        todo!()
    }

    pub fn max_with_shields() {
        todo!()
    }
}
