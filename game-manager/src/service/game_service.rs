use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::domain::error::database_error_to_response_error;
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

    pub async fn get_all_games(&self) -> Result<Vec<Game>, String> {
        self.game_repository.get_all().await.map_err(database_error_to_response_error)
    }
}