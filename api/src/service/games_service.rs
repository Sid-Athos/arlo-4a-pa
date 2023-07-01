
use axum::http::StatusCode;



use crate::database::repository::games_repository::GamesRepository;



use crate::domain::error::{database_error_to_status_code};
use crate::domain::model::games::Game;







pub struct GamesService {
    pub games_repository: GamesRepository,
}

impl GamesService {
    pub fn new(games_repository: GamesRepository) -> Self {
        GamesService {
            games_repository,
        }
    }

    pub async fn get_all_games(&self) -> Result<Vec<Game>, StatusCode> {
        println!("tamère1");
        self.games_repository.get_all_games().await.map_err(database_error_to_status_code)
    }
}