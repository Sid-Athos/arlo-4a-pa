use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lobby {
    pub id: i32,
    pub code: String,
    pub game_id: i32,
    pub private: bool,
    pub is_launched: bool,
    pub game_history_id : Option<i32>,
    pub from_move_history_id : Option<i32>
}
