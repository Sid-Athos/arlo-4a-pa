use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Game {
    pub id: i32,
    pub name:String,
    pub min_players: i32,
    pub max_players: i32,
    pub description: String
}