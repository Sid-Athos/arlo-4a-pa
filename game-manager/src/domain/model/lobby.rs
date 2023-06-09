use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lobby {
    pub id: i32,
    pub code: String,
    pub game_id: i32,
    pub private: bool,
}
