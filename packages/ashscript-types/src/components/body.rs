use enum_map::{enum_map, EnumMap};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{
        general::{UNIT_PART_COSTS, UNIT_PART_WEIGHTS},
        unit::{
            AGE_PER_GEN_PART, BATTERY_ENERGY_CAPACITY, CONVERT_COST, EXTRACT_PART_COST, GENERATOR_ENERGY_CAPACITY, GENERATOR_ENERGY_INCOME, UNIT_AGE_EXP, UNIT_BASE_AGE, UNIT_HEALTH_PER_PART, UNIT_SHIELD_HEALTH
        },
    },
    resource::Resource,
};

use super::shield::Shield;

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct UnitBody {
    pub parts: EnumMap<UnitPart, u32>,
    pub age: u32,
}

impl UnitBody {
    pub fn from_vec(parts: Vec<(UnitPart, u32)>) -> Self {
        let mut parts_map = enum_map! {
            _ => 0,
        };

        for (part, count) in parts {
            parts_map[part] += count;
        }

        Self {
            parts: parts_map,
            age: 0,
            // shield_health: parts_map[UnitPart::Shield] * UNIT_SHIELD_HEALTH,
        }
    }

    pub fn cost(&self) -> HashMap<Resource, u32> {
        let mut cost = HashMap::new();

        for (part_type, count) in self.parts.iter() {
            let (resource, cost_per_part) = UNIT_PART_COSTS[part_type];

            *cost.entry(resource).or_insert(0) += cost_per_part * count;
        }

        cost
    }

    pub fn weight(&self) -> f32 {
        let mut weight = 0.;

        for (part, count) in self.parts.iter() {
            weight += UNIT_PART_WEIGHTS[part] * (*count as f32);
        }

        weight
    }

    pub fn max_health_with_shields(&self, shield: Shield) -> u32 {
        self.max_health() + shield.health
    }

    pub fn max_health(&self) -> u32 {
        let mut health = 0;

        for (part, count) in self.parts.iter() {
            health += count * UNIT_HEALTH_PER_PART
        }

        health
    }

    pub fn max_shield_health(&self) -> u32 {
        self.parts[UnitPart::Shield] * UNIT_SHIELD_HEALTH
    }

    pub fn energy_income(&self) -> u32 {
        self.parts[UnitPart::Generate] * GENERATOR_ENERGY_INCOME
    }

    pub fn energy_capacity(&self) -> u32 {
        self.parts[UnitPart::Generate] * GENERATOR_ENERGY_CAPACITY
            + self.parts[UnitPart::Battery] * BATTERY_ENERGY_CAPACITY
    }

    pub fn max_age(&self) -> u32 {
        ((self.parts[UnitPart::Generate] * AGE_PER_GEN_PART) as f32).powf(UNIT_AGE_EXP) as u32
            + UNIT_BASE_AGE
    }

    pub fn range(&self) -> u32 {
        self.parts[UnitPart::Ranged]
    }

    pub fn damage(&self) -> u32 {
        self.parts[UnitPart::Ranged]
    }

    pub fn attack_cost(&self) -> u32 {
        self.parts[UnitPart::Ranged]
    }

    pub fn convert_speed(&self) -> u32 {
        self.parts[UnitPart::Convert]
    }

    pub fn convert_cost(&self) -> u32 {
        self.parts[UnitPart::Convert] * CONVERT_COST
    }

    pub fn fabricate_speed(&self) -> u32 {
        self.parts[UnitPart::Fabricate]
    }

    pub fn build_speed(&self) -> u32 {
        self.parts[UnitPart::Build]
    }

    pub fn build_efficiency(&self) -> u32 {
        self.parts[UnitPart::Build]
    }

    pub fn extract_speed(&self) -> u32 {
        self.parts[UnitPart::Extract]
    }

    pub fn extract_efficiency(&self) -> u32 {
        self.parts[UnitPart::Extract]
    }

    pub fn extract_cost(&self) -> u32 {
        self.parts[UnitPart::Extract] * EXTRACT_PART_COST
    }
}

#[derive(enum_map::Enum, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum UnitPart {
    Ranged,
    Harvest,
    Generate,
    Battery,
    Shield,
    Fabricate,
    Convert,
    Build,
    Extract,
    RangeImprovement,
    DamageImprovement,
    GenerateImprovement,
    BatteryImprovement,
    ShieldImprovement,
    FabricateImprovement,
    WeightImprovement,
}