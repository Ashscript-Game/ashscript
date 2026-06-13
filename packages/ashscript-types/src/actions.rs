use hashbrown::HashMap;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{components::body::UnitBody, objects::GameObjectKind, player::PlayerId, resource::Resource};

// REMINDER: These are intents that the server validates and wants executed

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ActionsByKind {
    pub unit_move: Vec<UnitMove>,
    pub unit_attack: Vec<UnitAttack>,
    pub turret_attack: Vec<TurretAttack>,
    pub factory_spawn_unit: Vec<FactorySpawnUnit>,
    pub unit_spawn_unit: Vec<UnitSpawnUnit>,
    pub resource_transfer: Vec<ResourceTransfer>,
    pub turret_repair: Vec<TurretRepair>,
    pub substation_collect: Vec<SubstationCollect>,
    pub extract_resource: Vec<ExtractResource>,
}

impl ActionsByKind {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitMove {
    pub from: Hex,
    pub to: Hex,
    pub cost: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitAttack {
    pub attacker_hex: Hex,
    pub target_hex: Hex,
    pub target_kind: GameObjectKind,
    pub cost: u32,
    pub damage: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurretAttack {
    pub turret_hex: Hex,
    pub target_hex: Hex,
    pub target_kind: GameObjectKind,
    pub damage: u32,
    pub cost: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactorySpawnUnit {
    pub factory_hex: Hex,
    pub out: Hex,
    pub body: UnitBody,
    pub name: String,
    pub cost: HashMap<Resource, u32>,
    pub owner: PlayerId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitSpawnUnit {
    pub parent_hex: Hex,
    pub out: Hex,
    pub body: UnitBody,
    pub name: String,
    pub cost: HashMap<Resource, u32>,
    pub owner: PlayerId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from: Hex,
    pub from_kind: GameObjectKind,
    pub to_hex: Hex,
    pub to_kind: GameObjectKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// Doesn't need to be used when the object is knowably destroyed due to other actions (such as being attacked such that health is at or below 0)
pub struct ObjectDestroyed {
    pub hex: Hex,
    pub kind: GameObjectKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitRechargeShield {
    pub unit_hex: Hex,
    pub amount: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurretRepair {
    pub turret_hex: Hex,
    pub target_hex: Hex,
    pub target_kind: GameObjectKind,
    pub repair: u32,
    pub cost: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubstationCollect {
    pub substation_hex: Hex,
    pub energy_collected: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtractResource {
    pub unit_hex: Hex,
    pub node_hex: Hex,
    pub amount: u32,
    pub cost: u32,
}