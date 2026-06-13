use std::default;

use serde::{Deserialize, Serialize};

#[derive(Default, enum_map::Enum, Hash, Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize,)]
pub enum Resource {
    #[default]
    Coal,
    Minerals,
    Scrap,
    Energy,
    Metal,
    Uranium,
}
