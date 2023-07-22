use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_members_history_repository::GameMembersHistoryRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::game_members_history::GameMembersHistory;
use crate::service::lobby_service::LobbyService;

pub struct GameMembersHistoryService {
    pub game_members_history_repository: GameMembersHistoryRepository,
    pub lobby_service: LobbyService
}

impl GameMembersHistoryService {

    pub fn new(pool: ConnectionPool) -> Self {
        GameMembersHistoryService {
            game_members_history_repository: GameMembersHistoryRepository::new(pool.clone()),
            lobby_service: LobbyService::new(pool)
        }
    }

    pub async fn create(&self, user_id: i32, game_history_id: i32, player: i32) -> Result<GameMembersHistory, StatusCode> {
        self.game_members_history_repository.create(user_id, game_history_id, player).await.map_err(database_error_to_status_code)
    }
}