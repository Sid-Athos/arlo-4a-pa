use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameStateResponse {
    scores: Vec<i32>,
    game_over: bool
}