use enum_map::{enum_map, EnumMap};
use hashbrown::HashSet;
use lazy_static::lazy_static;

use crate::{components::body::UnitPart, resource::Resource};

lazy_static! {
    pub static ref RESOURCE_INPUTS: EnumMap<Resource, HashSet<Resource>> = enum_map! {
        Resource::Metal => HashSet::from([Resource::Coal, Resource::Minerals]),
        Resource::Energy => HashSet::new(),
        Resource::Coal => HashSet::new(),
        Resource::Scrap => HashSet::new(),
        Resource::Minerals => HashSet::new(),
        Resource::Uranium => HashSet::new(),
    };
    pub static ref UNIT_PART_WEIGHTS: EnumMap<UnitPart, f32> = enum_map! {
        UnitPart::Ranged => 0.3,
        UnitPart::Generate => 0.2,
        UnitPart::Battery => 0.1,
        UnitPart::Harvest => 0.1,
        _ => 0.1,
    };
    pub static ref UNIT_PART_COSTS: EnumMap<UnitPart, (Resource, u32)> = enum_map! {
        UnitPart::Ranged => (Resource::Metal, 8),
        UnitPart::Generate => (Resource::Metal, 4),
        UnitPart::Battery => (Resource::Metal, 2),
        UnitPart::Harvest => (Resource::Metal, 3),
        _ => (Resource::Metal, 1),
    };
}

pub enum IntentReturnCode {
    InsufficientResources,
    WrongArgument,
    OutOfRange,
}

/// The length of a whole day (including night) in terms of ticks
pub const DAY_LENGTH: u64 = 250;
/// The length of time out of day that is night
pub const NIGHT_LENGTH: u64 = 80;