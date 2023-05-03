use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::model::game::Game;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GameResponse {
    pub id: i32,
    pub name: String,
    pub nb_player: i32,
}

impl GameResponse {

    pub fn from_domain(game: Game) -> Self {
        GameResponse {
            id: game.id,
            name: game.name,
            nb_player: game.nb_player,
        }
    }

    pub fn from_vec_domain(games: Vec<Game>) -> Vec<Self> {
        games.into_iter().map(|game| GameResponse::from_domain(game)).collect()
    }
}
