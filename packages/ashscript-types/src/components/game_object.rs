use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::objects::GameObjectKind;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct GameObject {
    id: Uuid,
    kind: GameObjectKind,
}