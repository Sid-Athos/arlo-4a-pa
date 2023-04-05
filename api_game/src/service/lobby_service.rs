use axum::http::StatusCode;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::lobby::Lobby;

pub struct LobbyService {
    pub lobby_repository: LobbyRepository,
}

impl LobbyService {

    pub fn new(lobby_repository: LobbyRepository) -> Self {
        LobbyService {
            lobby_repository,
        }
    }

    pub async fn get_public(&self) -> Result<Vec<Lobby>, StatusCode> {
        self.lobby_repository.get_public().await.map_err(database_error_to_status_code)
    }

    pub async fn get_public_by_game_id(&self, game_id: i32) -> Result<Vec<Lobby>, StatusCode> {
        self.lobby_repository.get_public_by_game_id(game_id).await.map_err(database_error_to_status_code)
    }
}