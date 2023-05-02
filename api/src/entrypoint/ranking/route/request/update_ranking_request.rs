use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateRankingRequest {
    pub p1_id: i32,
    pub p2_id: i32,
    pub game_id: i32,
    pub tie : bool,
    pub winner_id : i32
}