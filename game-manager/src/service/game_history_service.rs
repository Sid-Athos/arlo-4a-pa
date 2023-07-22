use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_history_repository::GameHistoryRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::game_history::GameHistory;

pub struct GameHistoryService {
    pub game_history_repository: GameHistoryRepository,
}

impl GameHistoryService {

    pub fn new(pool: ConnectionPool) -> Self {
        GameHistoryService {
            game_history_repository: GameHistoryRepository::new(pool.clone()),
        }
    }

    pub async fn get_all_by_user_id_and_game_id(&self, user_id: i32, game_id: i32) -> Result<Vec<GameHistory>, StatusCode> {
        self.game_history_repository.get_all_by_user_id_and_game_id(user_id, game_id).await.map_err(database_error_to_status_code)
    }
}