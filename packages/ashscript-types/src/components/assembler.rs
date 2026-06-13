use hashbrown::HashMap;
use hexx::Hex;
use ::serde::{Deserialize, Serialize};
use uuid::{serde, Uuid};

use crate::{objects::{GameObjectKind}, player::PlayerId};

pub type Assemblers = HashMap<Hex, Assembler>;

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct Assembler;