use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct Lobby {
    pub id: i32,
    pub code: String,
    pub game_id: i32,
    pub private: bool,
}
