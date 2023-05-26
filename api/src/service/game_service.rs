use axum::http::StatusCode;
use crate::database::repository::game_repository::GameRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::game::Game;
use crate::service::command::create_game_command::CreateGameCommand;

pub struct GameService {
    pub game_repository: GameRepository
}

impl GameService {
    pub fn new(game_repository: GameRepository) -> Self {
        GameService {
            game_repository
        }
    }
    pub async fn create_game(&self, mut game: CreateGameCommand) -> Result<Game, StatusCode> {
        self.game_repository.create_game(game.name, Option::from(game.description), game.min_players, game.max_players, game.path).await.map_err(database_error_to_status_code)
    }
}