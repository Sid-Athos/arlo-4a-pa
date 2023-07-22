use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameMoveHistory {
    pub id: i32,
    pub player : i32,
    pub game_state : String,
    pub action : String,
    pub action_number : i32,
    pub game_history_id : i32
}
