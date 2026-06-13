use std::any::TypeId;

use hecs::serialize::column::{
    deserialize_column, try_serialize, try_serialize_id, DeserializeContext, SerializeContext,
};
use hecs::{Archetype, ColumnBatchBuilder, ColumnBatchType, World};
use postcard::ser_flavors::{AllocVec, Flavor, HVec, Slice};
use serde::{Deserialize, Serialize};

use crate::components::assembler::Assembler;
use crate::components::body::UnitBody;
use crate::components::distributor::Distributor;
use crate::components::energy::Energy;
use crate::components::factory::Factory;
use crate::components::health::Health;
use crate::components::occupies::Occupies;
use crate::components::owner::Owner;
use crate::components::resource::{CoalNode, MineralNode, ResourceNode, UraniumNode};
use crate::components::shield::Shield;
use crate::components::solar_panel::SolarPanel;
use crate::components::spawn::Spawning;
use crate::components::storage::Storage;
use crate::components::substation::Substation;
use crate::components::terrain::{Lava, Plain, Terrain, Wall};
use crate::components::tile::Tile;
use crate::components::turbine::Turbine;
use crate::components::turret::Turret;
use crate::components::unit::Unit;
use crate::objects::GameObjectKind;

pub struct WorldWrapper(pub World);

fn serialize_archetypes() {}

// Identifiers for the components we want to include in the serialization process:
#[derive(Serialize, Deserialize)]
enum ComponentId {
    GameObjectKind,
    Owner,
    ResourceNode,
    CoalNode,
    MineralNode,
    UraniumNode,
    Shield,
    Spawning,
    Body,
    Energy,
    Storage,
    Terrain,
    Wall,
    Lava,
    Plain,
    Tile,
    Unit,
    Assembler,
    Distributor,
    Factory,
    Turret,
    Occupies,
    Health,
    Turbine,
    SolarPanel,
    Substation,
}

// We need to implement context types for the hecs serialization process:
#[derive(Default)]
struct SaveContextSerialize {}
#[derive(Default)]
struct SaveContextDeserialize {
    components: Vec<ComponentId>,
}
impl DeserializeContext for SaveContextDeserialize {
    fn deserialize_component_ids<'de, A>(&mut self, mut seq: A) -> Result<ColumnBatchType, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        self.components.clear();
        let mut batch = ColumnBatchType::new();
        while let Some(id) = seq.next_element()? {
            match id {
                ComponentId::GameObjectKind => {
                    batch.add::<GameObjectKind>();
                }
                ComponentId::Owner => {
                    batch.add::<Owner>();
                }
                ComponentId::ResourceNode => {
                    batch.add::<ResourceNode>();
                }
                ComponentId::CoalNode => {
                    batch.add::<CoalNode>();
                }
                ComponentId::MineralNode => {
                    batch.add::<MineralNode>();
                }
                ComponentId::UraniumNode => {
                    batch.add::<UraniumNode>();
                }
                ComponentId::Shield => {
                    batch.add::<Shield>();
                }
                ComponentId::Spawning => {
                    batch.add::<Spawning>();
                }
                ComponentId::Body => {
                    batch.add::<UnitBody>();
                }
                ComponentId::Energy => {
                    batch.add::<Energy>();
                }
                ComponentId::Storage => {
                    batch.add::<Storage>();
                }
                ComponentId::Terrain => {
                    batch.add::<Terrain>();
                }
                ComponentId::Wall => {
                    batch.add::<Wall>();
                }
                ComponentId::Lava => {
                    batch.add::<Lava>();
                }
                ComponentId::Plain => {
                    batch.add::<Plain>();
                }
                ComponentId::Tile => {
                    batch.add::<Tile>();
                }
                ComponentId::Unit => {
                    batch.add::<Unit>();
                }
                ComponentId::Turret => {
                    batch.add::<Turret>();
                }
                ComponentId::Factory => {
                    batch.add::<Factory>();
                }
                ComponentId::Assembler => {
                    batch.add::<Assembler>();
                }
                ComponentId::Distributor => {
                    batch.add::<Distributor>();
                }
                ComponentId::Occupies => {
                    batch.add::<Occupies>();
                }
                ComponentId::Health => {
                    batch.add::<Health>();
                }
                ComponentId::Turbine => {
                    batch.add::<Turbine>();
                }
                ComponentId::SolarPanel => {
                    batch.add::<SolarPanel>();
                }
                ComponentId::Substation => {
                    batch.add::<Substation>();
                }
            }
            self.components.push(id);
        }
        Ok(batch)
    }

    fn deserialize_components<'de, A>(
        &mut self,
        entity_count: u32,
        mut seq: A,
        batch: &mut ColumnBatchBuilder,
    ) -> Result<(), A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        for component in &self.components {
            match *component {
                ComponentId::GameObjectKind => {
                    deserialize_column::<GameObjectKind, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Owner => {
                    deserialize_column::<Owner, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::ResourceNode => {
                    deserialize_column::<ResourceNode, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::CoalNode => {
                    deserialize_column::<CoalNode, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::MineralNode => {
                    deserialize_column::<MineralNode, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::UraniumNode => {
                    deserialize_column::<UraniumNode, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Shield => {
                    deserialize_column::<Shield, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Spawning => {
                    deserialize_column::<Spawning, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Body => {
                    deserialize_column::<UnitBody, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Energy => {
                    deserialize_column::<Energy, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Storage => {
                    deserialize_column::<Storage, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Terrain => {
                    deserialize_column::<Terrain, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Wall => {
                    deserialize_column::<Wall, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Lava => {
                    deserialize_column::<Lava, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Plain => {
                    deserialize_column::<Plain, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Tile => {
                    deserialize_column::<Tile, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Unit => {
                    deserialize_column::<Unit, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Turret => {
                    deserialize_column::<Turret, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Factory => {
                    deserialize_column::<Factory, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Assembler => {
                    deserialize_column::<Assembler, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Distributor => {
                    deserialize_column::<Distributor, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Occupies => {
                    deserialize_column::<Occupies, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Health => {
                    deserialize_column::<Health, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Turbine => {
                    deserialize_column::<Turbine, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::SolarPanel => {
                    deserialize_column::<SolarPanel, _>(entity_count, &mut seq, batch)?;
                }
                ComponentId::Substation => {
                    deserialize_column::<Substation, _>(entity_count, &mut seq, batch)?;
                }
            }
        }
        Ok(())
    }
}

impl SerializeContext for SaveContextSerialize {
    fn component_count(&self, archetype: &Archetype) -> usize {
        archetype
            .component_types()
            .filter(|&t| {
                t == TypeId::of::<GameObjectKind>() || t == TypeId::of::<Owner>()
                    || t == TypeId::of::<ResourceNode>()
                    || t == TypeId::of::<CoalNode>()
                    || t == TypeId::of::<MineralNode>()
                    || t == TypeId::of::<UraniumNode>()
                    || t == TypeId::of::<Shield>()
                    || t == TypeId::of::<Spawning>()
                    || t == TypeId::of::<UnitBody>()
                    || t == TypeId::of::<Energy>()
                    || t == TypeId::of::<Storage>()
                    || t == TypeId::of::<Terrain>()
                    || t == TypeId::of::<Wall>()
                    || t == TypeId::of::<Lava>()
                    || t == TypeId::of::<Plain>()
                    || t == TypeId::of::<Tile>()
                    || t == TypeId::of::<Unit>()
                    || t == TypeId::of::<Turret>()
                    || t == TypeId::of::<Factory>()
                    || t == TypeId::of::<Assembler>()
                    || t == TypeId::of::<Distributor>()
                    || t == TypeId::of::<Occupies>()
                    || t == TypeId::of::<Health>()
                    || t == TypeId::of::<Turbine>()
                    || t == TypeId::of::<SolarPanel>()
                    || t == TypeId::of::<Substation>()
            })
            .count()
    }

    fn serialize_component_ids<S: serde::ser::SerializeTuple>(
        &mut self,
        archetype: &Archetype,
        mut out: S,
    ) -> Result<S::Ok, S::Error> {
        try_serialize_id::<GameObjectKind, _, _>(archetype, &ComponentId::GameObjectKind, &mut out)?;
        try_serialize_id::<Owner, _, _>(archetype, &ComponentId::Owner, &mut out)?;
        try_serialize_id::<ResourceNode, _, _>(archetype, &ComponentId::ResourceNode, &mut out)?;
        try_serialize_id::<CoalNode, _, _>(archetype, &ComponentId::CoalNode, &mut out)?;
        try_serialize_id::<MineralNode, _, _>(archetype, &ComponentId::MineralNode, &mut out)?;
        try_serialize_id::<UraniumNode, _, _>(archetype, &ComponentId::UraniumNode, &mut out)?;
        try_serialize_id::<Shield, _, _>(archetype, &ComponentId::Shield, &mut out)?;
        try_serialize_id::<Spawning, _, _>(archetype, &ComponentId::Spawning, &mut out)?;
        try_serialize_id::<UnitBody, _, _>(archetype, &ComponentId::Body, &mut out)?;
        try_serialize_id::<Energy, _, _>(archetype, &ComponentId::Energy, &mut out)?;
        try_serialize_id::<Storage, _, _>(archetype, &ComponentId::Storage, &mut out)?;
        try_serialize_id::<Terrain, _, _>(archetype, &ComponentId::Terrain, &mut out)?;
        try_serialize_id::<Wall, _, _>(archetype, &ComponentId::Wall, &mut out)?;
        try_serialize_id::<Lava, _, _>(archetype, &ComponentId::Lava, &mut out)?;
        try_serialize_id::<Plain, _, _>(archetype, &ComponentId::Plain, &mut out)?;
        try_serialize_id::<Tile, _, _>(archetype, &ComponentId::Tile, &mut out)?;
        try_serialize_id::<Unit, _, _>(archetype, &ComponentId::Unit, &mut out)?;
        try_serialize_id::<Turret, _, _>(archetype, &ComponentId::Turret, &mut out)?;
        try_serialize_id::<Factory, _, _>(archetype, &ComponentId::Factory, &mut out)?;
        try_serialize_id::<Assembler, _, _>(archetype, &ComponentId::Assembler, &mut out)?;
        try_serialize_id::<Distributor, _, _>(archetype, &ComponentId::Distributor, &mut out)?;
        try_serialize_id::<Occupies, _, _>(archetype, &ComponentId::Occupies, &mut out)?;
        try_serialize_id::<Health, _, _>(archetype, &ComponentId::Health, &mut out)?;
        try_serialize_id::<Turbine, _, _>(archetype, &ComponentId::Turbine, &mut out)?;
        try_serialize_id::<SolarPanel, _, _>(archetype, &ComponentId::SolarPanel, &mut out)?;
        try_serialize_id::<Substation, _, _>(archetype, &ComponentId::Substation, &mut out)?;
        out.end()
    }

    fn serialize_components<S: serde::ser::SerializeTuple>(
        &mut self,
        archetype: &Archetype,
        mut out: S,
    ) -> Result<S::Ok, S::Error> {
        try_serialize::<GameObjectKind, _>(archetype, &mut out)?;
        try_serialize::<Owner, _>(archetype, &mut out)?;
        try_serialize::<ResourceNode, _>(archetype, &mut out)?;
        try_serialize::<CoalNode, _>(archetype, &mut out)?;
        try_serialize::<MineralNode, _>(archetype, &mut out)?;
        try_serialize::<UraniumNode, _>(archetype, &mut out)?;
        try_serialize::<Shield, _>(archetype, &mut out)?;
        try_serialize::<Spawning, _>(archetype, &mut out)?;
        try_serialize::<UnitBody, _>(archetype, &mut out)?;
        try_serialize::<Energy, _>(archetype, &mut out)?;
        try_serialize::<Storage, _>(archetype, &mut out)?;
        try_serialize::<Terrain, _>(archetype, &mut out)?;
        try_serialize::<Wall, _>(archetype, &mut out)?;
        try_serialize::<Lava, _>(archetype, &mut out)?;
        try_serialize::<Plain, _>(archetype, &mut out)?;
        try_serialize::<Tile, _>(archetype, &mut out)?;
        try_serialize::<Unit, _>(archetype, &mut out)?;
        try_serialize::<Turret, _>(archetype, &mut out)?;
        try_serialize::<Factory, _>(archetype, &mut out)?;
        try_serialize::<Assembler, _>(archetype, &mut out)?;
        try_serialize::<Distributor, _>(archetype, &mut out)?;
        try_serialize::<Occupies, _>(archetype, &mut out)?;
        try_serialize::<Health, _>(archetype, &mut out)?;
        try_serialize::<Turbine, _>(archetype, &mut out)?;
        try_serialize::<SolarPanel, _>(archetype, &mut out)?;
        try_serialize::<Substation, _>(archetype, &mut out)?;
        out.end()
    }
}

pub fn serialize_world_data(world: &World) -> Vec<u8> {
    let mut buffer_vec: Vec<u8> = vec![];
    let buffer: &mut [u8] = buffer_vec.as_mut_slice();
    let buffer_storage: Slice = Slice::new(buffer);
    let mut hbufff: AllocVec<> = AllocVec::new();
    let _ = hbufff.try_extend(&buffer);
    let mut serializer = postcard::Serializer { output: hbufff };

    let _ = hecs::serialize::column::serialize(world, &mut SaveContextSerialize::default(), &mut serializer);

    // println!("buffer slice {:?}", buffer);
    // println!("buffer storage {:#?}", buffer_storage);
    // println!("serializer {:#?}", serializer);
    buffer_vec;

    serializer.output.finalize().ok().unwrap()
}

pub fn deserialize_world_data(
    world_data: Vec<u8>,
) -> Option<World> {

    let mut deserializer = postcard::Deserializer::from_bytes(&world_data);
    hecs::serialize::column::deserialize(&mut SaveContextDeserialize::default(), &mut deserializer).ok()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_serialize_world_data() {
        let mut world = World::default();
        world.spawn((Unit::default(), UnitBody::default(), Energy::default()));
        let data = serialize_world_data(&world);
        println!("resultant data {:#?}", data);
        
        println!("running test serialize_world_data");

    }
}

/* impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {

        World::deserialize(deserializer)
    }
}

impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.serialize(serializer)

    }
} */
