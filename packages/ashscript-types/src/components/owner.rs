use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Owner(pub Uuid);