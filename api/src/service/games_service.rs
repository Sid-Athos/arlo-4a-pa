use std::alloc::GlobalAlloc;
use axum::http::StatusCode;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use regex::{Regex, RegexSet};
use crate::database::repository::games_repository::GamesRepository;

use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::{database_error_to_status_code, internal_error};
use crate::domain::model::games::Game;
use crate::domain::model::session::Session;
use crate::domain::model::user::User;
use crate::service::command::change_password_command::ChangePasswordCommand;
use crate::service::command::create_user_command::CreateUserCommand;
use crate::service::command::login_command::LoginCommand;
use crate::service::command::updata_user_command::UpdateUserCommand;

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
        self.games_repository.get_all_games().await.map_err(database_error_to_status_code)
    }
}