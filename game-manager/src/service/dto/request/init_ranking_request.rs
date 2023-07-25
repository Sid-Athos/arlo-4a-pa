use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InitRankingRequest {
    pub user_id: i32,
    pub game_id: i32,
}

impl InitRankingRequest {
    pub fn new(user_id: i32, game_id: i32) -> Self {
        InitRankingRequest {
            user_id,
            game_id
        }
    }
}