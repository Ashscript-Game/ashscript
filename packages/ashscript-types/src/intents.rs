use hecs::Entity;
use hexx::Hex;

use crate::{components::body::UnitBody, objects::GameObjectKind, player::PlayerId, resource::Resource};

// REMINDER: These are player-generated intents

pub type Intents = Vec<Intent>;

pub enum Intent {
    UnitMove(UnitMove),
    UnitAttack(UnitAttack),
    TurretAttack(TurretAttack),
    FactorySpawnUnit(FactorySpawnUnit),
    UnitSpawnUnit(UnitSpawnUnit),
    ResourceTransfer(ResourceTransfer),
    TurretRepair(TurretRepair),
}

#[derive(enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum IntentName {
    UnitMove,
    UnitAttack,
    TurretAttack,
    FactorySpawnUnit,
    ResourceTransfer,
}


/// A unit moving from one hex to another
pub struct UnitMove {
    pub from: Hex,
    pub to: Hex,
}

/// A unit attacking an attackable target
pub struct UnitAttack {
    pub attacker_hex: Hex,
    pub target_hex: Hex,
    pub target_kind: GameObjectKind,
}

/// A turret attacking an attackable target
pub struct TurretAttack {
    pub turret_hex: Hex,
    pub target_hex: Hex,
    pub target_kind: GameObjectKind,
}

/// The spawning of a unit from a factory
pub struct FactorySpawnUnit {
    pub factory_hex: Hex,
    pub body: UnitBody,
    pub name: String,
    /// If out hexes are not provided the engine will choose the first empty one in a clockwise direction
    /// If out hexes are provided, sucessfully spawned units will be outputed to the first empty hex 
    pub out: Option<Vec<Hex>>,
    pub owner: PlayerId,
}

/// The spawning of a unit from a unit
pub struct UnitSpawnUnit {
    pub unit_hex: Hex,
    pub body: UnitBody,
    pub name: String,
    /// If out hexes are not provided the engine will choose the first empty one in a clockwise direction
    /// If out hexes are provided, sucessfully spawned units will be outputed to the first empty hex 
    pub out: Option<Vec<Hex>>,
    pub owner: PlayerId,
}

/// A resource transfer from one storable object to another
pub struct ResourceTransfer {
    pub resource: Resource,
    pub amount: u32,
    pub from_hex: Hex,
    pub to_hex: Hex,
    pub from_kind: GameObjectKind,
    pub to_kind: GameObjectKind,
}

pub struct UnitRechargeShield {
    pub unit_hex: Hex,
    pub amount: Option<u32>,
}

pub struct TurretRepair {
    pub turret_hex: Hex,
    pub target_hex: Hex,
    pub target_kind: GameObjectKind,
}

pub struct ExtractResource {
    pub unit_hex: Hex,
    pub node_hex: Hex,
}