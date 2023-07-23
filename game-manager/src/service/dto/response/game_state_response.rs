use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameStateResponse {
    pub scores: Vec<i32>,
    pub game_over: bool
}