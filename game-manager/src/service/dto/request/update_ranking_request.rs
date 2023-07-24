use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateRankingRequest {
    pub winner_id: i32,
    pub losers_id: Vec<i32>,
    pub game_id: i32,
    pub tie : bool,
    pub average_rank : i32
}

impl UpdateRankingRequest {
    pub fn new(winner_id: i32, losers_id: Vec<i32>, game_id : i32, tie : bool, average_rank : i32) -> Self {
        UpdateRankingRequest {
            winner_id,
            losers_id,
            game_id,
            tie,
            average_rank
        }
    }
}