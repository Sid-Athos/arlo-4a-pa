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
}