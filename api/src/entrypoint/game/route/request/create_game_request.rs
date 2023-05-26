use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateGameRequest {
    pub name: String,
    pub min_players : i32,
    pub max_players : i32,
    pub description : String,
    pub path : String
}