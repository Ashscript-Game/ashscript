use serde::{Deserialize, Serialize};

use crate::resource::Resource;


#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ResourceNode {
    pub amount: u32,
    pub resource: Resource,
}

impl ResourceNode {
    pub fn new(amount: u32, resource: Resource) -> Self {
        Self {
            amount,
            resource,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct CoalNode;
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MineralNode;
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct UraniumNode;