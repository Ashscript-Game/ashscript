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

    /// A compact, per-kind count of the actions in this batch.
    ///
    /// Used for concise structured logging on the server and the live actions
    /// readout on the client, instead of dumping every action with `{:?}`.
    pub fn counts(&self) -> ActionCounts {
        ActionCounts {
            unit_move: self.unit_move.len(),
            unit_attack: self.unit_attack.len(),
            turret_attack: self.turret_attack.len(),
            factory_spawn_unit: self.factory_spawn_unit.len(),
            unit_spawn_unit: self.unit_spawn_unit.len(),
            resource_transfer: self.resource_transfer.len(),
            turret_repair: self.turret_repair.len(),
            substation_collect: self.substation_collect.len(),
            extract_resource: self.extract_resource.len(),
        }
    }
}

/// Per-kind counts of an [`ActionsByKind`] batch. Cheap to copy and log.
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ActionCounts {
    pub unit_move: usize,
    pub unit_attack: usize,
    pub turret_attack: usize,
    pub factory_spawn_unit: usize,
    pub unit_spawn_unit: usize,
    pub resource_transfer: usize,
    pub turret_repair: usize,
    pub substation_collect: usize,
    pub extract_resource: usize,
}

impl ActionCounts {
    /// Total number of actions across all kinds.
    pub fn total(&self) -> usize {
        self.unit_move
            + self.unit_attack
            + self.turret_attack
            + self.factory_spawn_unit
            + self.unit_spawn_unit
            + self.resource_transfer
            + self.turret_repair
            + self.substation_collect
            + self.extract_resource
    }

    /// `(label, count)` pairs in a stable order, for tabular/HUD rendering.
    pub fn entries(&self) -> [(&'static str, usize); 9] {
        [
            ("move", self.unit_move),
            ("attack", self.unit_attack),
            ("turret_attack", self.turret_attack),
            ("factory_spawn", self.factory_spawn_unit),
            ("unit_spawn", self.unit_spawn_unit),
            ("transfer", self.resource_transfer),
            ("turret_repair", self.turret_repair),
            ("substation_collect", self.substation_collect),
            ("extract", self.extract_resource),
        ]
    }
}

impl std::fmt::Display for ActionCounts {
    /// Compact, only-nonzero rendering, e.g. `move=3 attack=2 unit_spawn=1`.
    /// Renders `none` when there are no actions.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut wrote = false;
        for (label, count) in self.entries() {
            if count == 0 {
                continue;
            }
            if wrote {
                f.write_str(" ")?;
            }
            write!(f, "{label}={count}")?;
            wrote = true;
        }
        if !wrote {
            f.write_str("none")?;
        }
        Ok(())
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