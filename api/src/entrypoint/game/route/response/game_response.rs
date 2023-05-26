use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::model::game::Game;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GameResponse {
    pub id: i32,
    pub name: String,
    pub max_players: i32,
    pub min_players: i32,
    pub description : String,
    pub path : String
}

impl GameResponse {
    pub fn from_domain(game: Game) -> Self {
        GameResponse {
            id: game.id,
            name: game.name,
            max_players: game.max_players,
            min_players: game.min_players,
            description: game.description,
            path : game.path
        }
    }

    pub fn from_vec_domain(friend_lists: Vec<Game>) -> Vec<Self> {
        friend_lists.into_iter().map(|friend| GameResponse::from_domain(friend)).collect()
    }
}