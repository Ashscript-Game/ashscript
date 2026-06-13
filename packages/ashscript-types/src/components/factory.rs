use hashbrown::HashMap;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    components::body::{UnitBody, UnitPart}, constants::general::{IntentReturnCode, UNIT_PART_COSTS}, intents::{self, Intent, Intents}, objects::GameObjectKind, player::PlayerId
};

pub type Factories = HashMap<Hex, Factory>;

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct Factory;

pub fn new_factory() {

}

/* impl Factory {
    pub fn spawn_unit(
        &self,
        name: String,
        parts: Vec<(UnitPart, u32)>,
        outs: Option<Vec<Hex>>,
        intents: &mut Intents,
    ) {
        intents.push(Intent::FactorySpawnUnit(intents::FactorySpawnUnit {
            factory_hex: self.hex,
            name,
            body: UnitBody::from_vec(parts),
            out: outs,
        }));
    }

    pub fn can_spawn_body(&self, parts: &[(UnitPart, u32)]) -> bool {
        for (part, part_amount) in parts.iter() {
            let (resource, resource_amount) = UNIT_PART_COSTS[*part];

            if !self
                .storage
                .has_sufficient(&resource, &(resource_amount * part_amount))
            {
                return false;
            }
        }

        true
    }

    pub fn spawn_unit_checked(
        &self,
        name: String,
        parts: Vec<(UnitPart, u32)>,
        outs: Option<Vec<Hex>>,
        intents: &mut Intents,
    ) -> Result<(), IntentReturnCode> {
        if !self.can_spawn_body(&parts) {
            return Err(IntentReturnCode::InsufficientResources);
        }

        // We passed all the checks, create the intent

        self.spawn_unit(name, parts, outs, intents);

        Ok(())
    }
}
 */