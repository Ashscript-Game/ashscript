use enum_map::{enum_map, EnumMap};
use hashbrown::HashMap;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    components::body::UnitBody, constants::{
        general::{UNIT_PART_COSTS, UNIT_PART_WEIGHTS},
        unit::{AGE_PER_GEN_PART, BATTERY_ENERGY_CAPACITY, GENERATOR_ENERGY_CAPACITY, GENERATOR_ENERGY_INCOME, UNIT_AGE_EXP, UNIT_BASE_AGE, UNIT_HEALTH_PER_PART, UNIT_SHIELD_HEALTH},
    }, intents::{self, Intent, Intents}, objects::GameObjectKind, player::{Player, PlayerId}, resource::Resource
};

pub type Units = HashMap<Hex, Unit>;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Unit {
    pub name: String,
}

impl Unit {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}
