use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InitRequest {
    pub init: PlayerRequest,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerRequest {
    pub players: i32,
}

impl InitRequest {
    pub fn new(player: i32) -> Self {
        InitRequest {
            init: PlayerRequest {
                players: player
            }
        }
    }
}