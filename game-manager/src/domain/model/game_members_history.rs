use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameMembersHistory {
    pub id: i32,
    pub user_id : i32,
    pub game_history_id : i32,
    pub player : i32
}
