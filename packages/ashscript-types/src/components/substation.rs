use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct Substation {}

impl Substation {
    pub fn new() -> Self {
        Self {}
    }

    pub fn range(&self) -> u32 {
        6
    }
}