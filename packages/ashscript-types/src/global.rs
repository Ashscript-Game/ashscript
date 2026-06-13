use std::time::Duration;

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::{
    constants::general::{DAY_LENGTH, NIGHT_LENGTH},
    player::{Player, PlayerId},
};

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Global {
    pub tick: u64,
    pub last_tick_duration: Duration,
    pub players: HashMap<PlayerId, Player>,
}

impl Global {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn is_day(&self) -> bool {
        (self.tick + NIGHT_LENGTH) % DAY_LENGTH > NIGHT_LENGTH
    }

    pub fn is_night(&self) -> bool {
        !self.is_day()
    }

    /// Returns none if it is night
    pub fn time_to_night(&self) -> Option<u64> {
        let time_to = (self.tick + NIGHT_LENGTH) % DAY_LENGTH;
        if time_to < NIGHT_LENGTH {
            return None
        }

        Some(time_to)
    }
}
