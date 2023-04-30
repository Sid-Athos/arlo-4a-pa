use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateLobbyRequest {
    pub game_id: i32,
    pub private: bool,
}