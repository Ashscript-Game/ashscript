use enum_iterator::Sequence;
use enum_map::Enum;
use serde::{Deserialize, Serialize};

/// Each type has its own storage inside of a chunk, and cannot share a hex/tile with another of its own type (ex: two units cannot be on the same tile)
/// Not exactly sure if this is useful anywhere
#[derive(Serialize, Deserialize, Clone, Copy, Default, Eq, PartialEq, Hash, Debug, Sequence, Enum)]
pub enum GameObjectKind {
    Unit,
    Turret,
    Factory,
    Distributor,
    Assembler,
    Terrain,
    Turbine,
    SolarPanel,
    Substation,
    ResourceNode,
    #[default]
    Unknown,
}
