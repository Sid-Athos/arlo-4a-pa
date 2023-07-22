use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameHistory {
    pub id: i32,
    pub date_time : String,
    pub nb_players : i32,
    pub game_id : i32
}
