use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::model::game::Game;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct GameResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub min_players: i32,
    pub max_players: i32,
}

impl GameResponse {

    pub fn from_domain(game: Game) -> Self {
        GameResponse {
            id: game.id,
            name: game.name,
            description: game.description,
            min_players: game.min_players,
            max_players: game.max_players,
        }
    }

    pub fn from_vec_domain(games: Vec<Game>) -> Vec<Self> {
        games.into_iter().map(|game| GameResponse::from_domain(game)).collect()
    }
}
