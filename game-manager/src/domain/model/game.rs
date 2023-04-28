use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub nb_player: i32,
}
