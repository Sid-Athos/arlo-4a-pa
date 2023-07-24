use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Id {
    id: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ranking {
    pub id: i32,
    pub rank : i32,
    pub user_id : i32,
    pub game_id : i32,
    pub nb_games : i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RankingResponse {
    pub id: i32,
    pub rank : i32,
    pub user: Id,
    pub game : Id,
    pub nb_games : i32
}

impl RankingResponse {
    pub async fn new(rankingResponse : RankingResponse) -> Ranking {
        Ranking {
            id : rankingResponse.id,
            rank : rankingResponse.rank,
            user_id : rankingResponse.user.id,
            game_id : rankingResponse.game.id,
            nb_games : rankingResponse.nb_games
        }
    }
}