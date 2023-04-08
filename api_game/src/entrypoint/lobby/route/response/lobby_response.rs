use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::model::lobby::Lobby;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LobbyResponse {
    pub id: i32,
    pub code: String,
    pub game_id: i32,
    pub private: bool,
}

impl LobbyResponse {

    pub fn from_domain(lobby: Lobby) -> Self {
        LobbyResponse {
            id: lobby.id,
            code: lobby.code,
            game_id: lobby.game_id,
            private: lobby.private,
        }
    }

    pub fn from_vec_domain(lobbies: Vec<Lobby>) -> Vec<Self> {
        lobbies.into_iter().map(|lobby| LobbyResponse::from_domain(lobby)).collect()
    }
}