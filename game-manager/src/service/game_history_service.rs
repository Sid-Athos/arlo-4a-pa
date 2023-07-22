use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_history_repository::GameHistoryRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::game_history::GameHistory;
use crate::service::lobby_service::LobbyService;

pub struct GameHistoryService {
    pub game_history_repository: GameHistoryRepository,
    pub lobby_service: LobbyService
}

impl GameHistoryService {

    pub fn new(pool: ConnectionPool) -> Self {
        GameHistoryService {
            game_history_repository: GameHistoryRepository::new(pool.clone()),
            lobby_service: LobbyService::new(pool)
        }
    }

    pub async fn get_all_by_user_id_and_game_id(&self, user_id: i32, game_id: i32) -> Result<Vec<GameHistory>, StatusCode> {
        self.game_history_repository.get_all_by_user_id_and_game_id(user_id, game_id).await.map_err(database_error_to_status_code)
    }

    pub async fn create(&self, lobby_id: i32) -> Result<GameHistory, StatusCode> {
        let lobby = self.lobby_service.get_by_id(lobby_id).await?;
        let members = self.lobby_service.get_lobby_member(lobby_id).await?;
        self.game_history_repository.create(members.len() as i32, lobby.game_id).await.map_err(database_error_to_status_code)
    }
}