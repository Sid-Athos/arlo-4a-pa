use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RankingResponse {
    pub id: i32,
    pub rank : i32,
    pub nb_games : i32
}