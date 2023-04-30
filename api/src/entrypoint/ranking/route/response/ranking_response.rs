use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::model::ranking::Ranking;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct RankingResponse {
    pub id: i32,
    pub rank : i32,
    pub user_id : i32,
    pub game_id : i32,
    pub nb_games : i32
}

impl RankingResponse {
    pub fn from_domain(ranking: Ranking) -> Self {
        RankingResponse {
            id: ranking.id,
            rank: ranking.rank,
            user_id: ranking.user_id,
            game_id: ranking.game_id,
            nb_games: ranking.nb_games
        }
    }

    pub fn from_vec_domain(rankings: Vec<Ranking>) -> Vec<Self> {
        rankings.into_iter().map(|ranking| RankingResponse::from_domain(ranking)).collect()
    }
}