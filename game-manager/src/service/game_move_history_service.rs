use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_move_history_repository::GameMoveHistoryRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::game_move_history::GameMoveHistory;

pub struct GameMoveHistoryService {
    pub game_move_history_repository: GameMoveHistoryRepository,
}

impl GameMoveHistoryService {

    pub fn new(pool: ConnectionPool) -> Self {
        GameMoveHistoryService {
            game_move_history_repository: GameMoveHistoryRepository::new(pool.clone()),
        }
    }

    pub async fn get_all_by_game_history_id(&self, game_history_id: i32) -> Result<Vec<GameMoveHistory>, StatusCode> {
        self.game_move_history_repository.get_all_by_game_history_id(game_history_id).await.map_err(database_error_to_status_code)
    }
}