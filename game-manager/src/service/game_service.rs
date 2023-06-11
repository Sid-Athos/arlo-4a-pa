use std::fs::File;
use std::io::Write;
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::domain::error::database_error_to_response_error;
use crate::domain::model::game::Game;
use crate::service::command::update_game_command::UpdateGameCommand;

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

    pub async fn create_game(&self, name : String, max_players : i32, min_players : i32, description : Option<String>, language : String, code : String, user_id : i32)-> Result<Game, String>{
        let game = self.game_repository.create(name,min_players,max_players,description,language,user_id, code.clone()).await.map_err(database_error_to_response_error)?;

        let mut f = File::create(format!("ressources/games/{}.{}",game.id,game.language)).map_err(
            |_|"INTERNAL_SERVER_ERROR".to_string()
        )?;
        f.write_all(code.as_bytes()).map_err(
            |_|"INTERNAL_SERVER_ERROR".to_string()
        )?;
        Ok(game)
    }

    pub async fn get_game_by_id(&self, id : i32, user_id : i32)-> Result<Game, String>{
        let game  = self.game_repository.get_game_and_code_by_id(id).await.map_err(database_error_to_response_error)?;
        if game.user_id == user_id {
            Ok(game)
        }else{
            self.game_repository.get_by_id(id).await.map_err(database_error_to_response_error)
        }
    }

    pub async fn delete_by_user(&self, id: i32, user_id : i32) -> Result<Game, String> {
        let game = self.game_repository.get_by_id(id).await.map_err(database_error_to_response_error)?;
        if game.user_id == user_id {
            self.game_repository.delete(id).await.map_err(database_error_to_response_error)
        }else {
            Ok(game) //TODO a voir avec arnaud pour les erreurs
        }
    }

    pub async fn delete_by_admin(&self, id: i32) -> Result<Game, String> {
        self.game_repository.delete(id).await.map_err(database_error_to_response_error)
    }

    pub async fn update_game(&self, update_game_command: UpdateGameCommand, user_id : i32) -> Result<Game, String> {
        let mut game = self.game_repository.get_by_id(update_game_command.id).await.map_err(database_error_to_response_error)?;


        if update_game_command.name.is_some() {
            game = self.game_repository.update_name(update_game_command.name.unwrap_or_default(), update_game_command.id).await.map_err(database_error_to_response_error)?;
        }

        if update_game_command.description.is_some() {
            game = self.game_repository.update_description(update_game_command.description.unwrap_or_default(), update_game_command.id).await.map_err(database_error_to_response_error)?;
        }

        if update_game_command.language.is_some() {
            game = self.game_repository.update_language(update_game_command.language.unwrap_or_default(), update_game_command.id).await.map_err(database_error_to_response_error)?;
        }

        if update_game_command.max_players.is_some() {
            game = self.game_repository.update_max_players(update_game_command.max_players.unwrap_or_default(), update_game_command.id).await.map_err(database_error_to_response_error)?;
        }

        if update_game_command.min_players.is_some() {
            game = self.game_repository.update_min_players(update_game_command.min_players.unwrap_or_default(), update_game_command.id).await.map_err(database_error_to_response_error)?;
        }

        if update_game_command.code.is_some() && user_id == game.user_id {
            game = self.game_repository.update_code(update_game_command.code.clone().unwrap_or_default(), update_game_command.id).await.map_err(database_error_to_response_error)?;
            let mut f = File::create(format!("ressources/games/{}.{}",game.id,game.language)).map_err(
                |_|"INTERNAL_SERVER_ERROR".to_string()
            )?;
            let code = update_game_command.code.unwrap();
            f.write_all(code.as_bytes()).map_err(
                |_|"INTERNAL_SERVER_ERROR".to_string()//TODO a voir avec arnaud pour les erreurs
            )?;
        }

        Ok(game)
    }

}