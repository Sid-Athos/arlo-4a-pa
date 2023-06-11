use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateGameRequest {
    pub name: Option<String>,
    pub min_players : Option<i32>,
    pub max_players : Option<i32>,
    pub description : Option<String>,
    pub code : Option<String>,
    pub language : Option<String>
}