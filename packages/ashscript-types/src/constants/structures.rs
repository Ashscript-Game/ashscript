use enum_map::EnumMap;
use hashbrown::{HashMap, HashSet};
use lazy_static::lazy_static;

use crate::objects::GameObjectKind;

lazy_static! {
    pub static ref IMPASSIBLE_GAME_OBJECTS: HashSet<GameObjectKind> = [
        GameObjectKind::Turret,
        GameObjectKind::Factory,
        GameObjectKind::Unit
    ]
    .iter()
    .cloned()
    .collect::<HashSet<GameObjectKind>>();
    pub static ref GAME_OBJECT_HEALTHS: HashMap<GameObjectKind, u32> = HashMap::from_iter([
        (GameObjectKind::Turret, 100),
        (GameObjectKind::Factory, 100),
        (GameObjectKind::Distributor, 100),
        (GameObjectKind::Assembler, 100)
    ]);

    pub static ref GAME_OBJECT_ENERGY_CAPACITIES: HashMap<GameObjectKind, u32> = HashMap::from_iter([
        (GameObjectKind::Turret, 10_000),
        (GameObjectKind::Distributor, 10_000),
        (GameObjectKind::Substation, 100_000),
    ]);

    
}

pub const WIND_OUTPUT: u32 = 10;
pub const SOLAR_OUTPUT: u32 = 80;