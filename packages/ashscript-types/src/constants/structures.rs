use std::sync::LazyLock;

use hashbrown::{HashMap, HashSet};

use crate::objects::GameObjectKind;

pub static IMPASSIBLE_GAME_OBJECTS: LazyLock<HashSet<GameObjectKind>> = LazyLock::new(|| {
    [
        GameObjectKind::Turret,
        GameObjectKind::Factory,
        GameObjectKind::Unit,
    ]
    .iter()
    .cloned()
    .collect::<HashSet<GameObjectKind>>()
});

pub static GAME_OBJECT_HEALTHS: LazyLock<HashMap<GameObjectKind, u32>> = LazyLock::new(|| {
    HashMap::from_iter([
        (GameObjectKind::Turret, 100),
        (GameObjectKind::Factory, 100),
        (GameObjectKind::Distributor, 100),
        (GameObjectKind::Assembler, 100),
    ])
});

pub static GAME_OBJECT_ENERGY_CAPACITIES: LazyLock<HashMap<GameObjectKind, u32>> =
    LazyLock::new(|| {
        HashMap::from_iter([
            (GameObjectKind::Turret, 10_000),
            (GameObjectKind::Distributor, 10_000),
            (GameObjectKind::Substation, 100_000),
        ])
    });

pub const WIND_OUTPUT: u32 = 10;
pub const SOLAR_OUTPUT: u32 = 80;