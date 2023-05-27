use std::fs::File;
use std::io::Write;
use axum::http::StatusCode;
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

    pub async fn create_game(&self, name : String, max_players : i32, min_players : i32, description : String, language : String, file : String, user_id : i32)-> Result<Game, String>{
        let game = self.game_repository.create(name,min_players,max_players,description,language,user_id).await.map_err(database_error_to_response_error)?;

        let mut f = File::create(format!("ressources/games/{}.{}",game.id,game.language)).map_err(
            |_|"INTERNAL_SERVER_ERROR".to_string()
        )?;
        f.write_all(file.as_bytes()).map_err(
            |_|"INTERNAL_SERVER_ERROR".to_string()
        )?;
        Ok(game)
    }
}