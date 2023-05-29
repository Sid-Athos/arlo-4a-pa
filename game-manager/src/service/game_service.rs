use tokio_tungstenite::tungstenite::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::domain::error::{database_error_to_response_error, database_error_to_status_code};
use crate::domain::model::game::Game;

pub struct GameService {
    pub game_repository: GameRepository,
}

impl GameService {

    pub fn new(pool: ConnectionPool) -> Self {
        GameService {
            game_repository: GameRepository::new(pool.clone()),
        }
    }

    pub async fn get_all_games(&self) -> Result<Vec<Game>, StatusCode> {
        self.game_repository.get_all().await.map_err(database_error_to_status_code)
    }

    pub async fn get_by_id(&self, game_id: i32) -> Result<Game, StatusCode> {
        self.game_repository.get_by_id(game_id).await.map_err(database_error_to_status_code)
    }
}