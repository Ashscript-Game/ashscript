use std::any::TypeId;

use hashbrown::{HashMap, HashSet};
use hecs::Entity;
use hexx::{shapes::hexagon, Hex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    chunk::{Chunk, Chunks},
    constants::map::CHUNK_SIZE,
    entity::Component,
    objects::GameObjectKind,
};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    pub chunks: Chunks,
    pub data: MapData,
}

impl Map {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn chunk_at(&self, hex: &Hex) -> Option<&Chunk> {
        let chunk_hex = hex.to_lower_res(CHUNK_SIZE);
        self.chunks.get(&chunk_hex)
    }

    pub fn chunk_at_mut(&mut self, hex: &Hex) -> Option<&mut Chunk> {
        let chunk_hex = hex.to_lower_res(CHUNK_SIZE);
        self.chunks.get_mut(&chunk_hex)
    }

    pub fn entity_at(&self, hex: &Hex, kind: GameObjectKind) -> Option<&Entity> {
        let chunk = self.chunk_at(hex)?;
        chunk.entities[kind].get(hex)
    }

    pub fn remove_entity(&mut self, hex: &Hex, kind: GameObjectKind) -> Option<Entity> {
        let chunk = self.chunk_at_mut(hex)?;
        chunk.entities[kind].remove(hex)
    }

    pub fn chunks_in_area(&mut self, hex: Hex, radius: u32) -> Vec<&Chunk> {
        let mut found_chunk_hexes = HashSet::new();
        let mut chunks = Vec::new();

        for search_hex in hexagon(hex, radius) {
            let chunk_hex = search_hex.to_lower_res(CHUNK_SIZE);
            if found_chunk_hexes.contains(&chunk_hex) {
                continue;
            }

            found_chunk_hexes.insert(chunk_hex);

            if let Some(chunk) = self.chunks.get(&chunk_hex) {
                chunks.push(chunk);
            }
        }

        chunks
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct MapData {
    pub radius: u32,
}

impl MapData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
