use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateGameRequest {
    pub name: String,
    pub min_players : i32,
    pub max_players : i32,
    pub description : Option<String>,
    pub code : String,
    pub language : String
}