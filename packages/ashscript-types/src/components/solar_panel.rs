use hexx::Hex;
use serde::{Deserialize, Serialize};

use crate::{chunk::Chunk, constants::structures::{SOLAR_OUTPUT, WIND_OUTPUT}, map::Map};

#[derive(Serialize, Default, Deserialize, Clone, Debug)]
pub struct SolarPanel {}

impl SolarPanel {
    pub fn output(&self, map: &Map, hex: &Hex) -> Option<u32> {
        let chunk = map.chunk_at(hex)?;

        Some(self.output_chunk(chunk))
    }   

    pub fn output_chunk(&self, chunk: &Chunk) -> u32 {
        (chunk.solar_efficiency * (SOLAR_OUTPUT as f32)) as u32
    }
}