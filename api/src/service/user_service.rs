use bcrypt::{hash, verify};
use axum::http::StatusCode;
use rand::distributions::Alphanumeric;
use rand::{Rng, thread_rng};
use crate::database::database_error::DatabaseError;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::{database_error_to_status_code, internal_error};
use crate::domain::model::session::Session;
use crate::domain::model::user::User;
use crate::service::command::change_password_command::ChangePasswordCommand;
use crate::service::command::create_user_command::CreateUserCommand;
use crate::service::command::login_command::LoginCommand;
use crate::service::command::update_user_command::UpdateUserCommand;

pub struct UserService {
    pub user_repository: UserRepository,
    pub session_repository: SessionRepository,
}

impl UserService {

    pub fn new(user_repository: UserRepository, session_repository: SessionRepository) -> Self {
        UserService {
            user_repository,
            session_repository,
        }
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<User, StatusCode> {
        self.user_repository.get_user_by_id(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn create_user(&self, mut user: CreateUserCommand) -> Result<User, StatusCode> {

        user.password = hash(user.password, 4).map_err(internal_error)?;

        self.user_repository.create_user(user).await.map_err(database_error_to_status_code)
    }

    pub async fn login_user(&self, user: LoginCommand) -> Result<Session, StatusCode> {
        let user_bdd = self.user_repository.get_user_by_email(user.email).await.map_err(database_error_to_status_code)?;

        if verify(user.password, &user_bdd.password).map_err(internal_error)? {

            let token: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(256)
                .map(char::from)
                .collect();

            self.session_repository.create_session(user_bdd.id, token).await.map_err(database_error_to_status_code)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    pub async fn search_user(&self, search: String) -> Result<Vec<User>, StatusCode> {
        self.user_repository.search_user(search).await.map_err(database_error_to_status_code)
    }

    pub async fn delete_account(&self, user_id: i32) -> Result<User, StatusCode> {
        self.session_repository.delete_all_for_user(user_id).await.map_err(database_error_to_status_code)?;
        self.user_repository.delete_account(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn change_password(&self, change_password_command: ChangePasswordCommand) -> Result<User, StatusCode> {
        let user_bdd = self.user_repository.get_user_by_id(change_password_command.user_id).await.map_err(database_error_to_status_code)?;

        if verify(change_password_command.old_password, &user_bdd.password).map_err(internal_error)? {
            let new_password = hash(change_password_command.new_password, 4).map_err(internal_error)?;
            self.user_repository.change_password(change_password_command.user_id, new_password).await.map_err(database_error_to_status_code)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    pub async fn update_user(&self, update_user_command: UpdateUserCommand) -> Result<User, StatusCode> {

        let user = self.user_repository.get_user_by_pseudo(update_user_command.pseudo.clone()).await;

        match user {
            Ok(_) => {
                return Err(StatusCode::CONFLICT);
            }
            Err(e) => {
                match e {
                    DatabaseError::NotFound => {}
                    _ => {
                        return Err(database_error_to_status_code(e));
                    }
                }
            }
        }

        self.user_repository.update_user(update_user_command).await.map_err(database_error_to_status_code)
    }
}