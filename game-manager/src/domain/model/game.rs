use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub min_players: i32,
    pub max_players: i32,
    pub language: String
}
