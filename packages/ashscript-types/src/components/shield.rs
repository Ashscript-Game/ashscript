use serde::{Deserialize, Serialize};


#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Shield {
    pub health: u32,
    pub max_health: u32,
    pub regen: u32,
}