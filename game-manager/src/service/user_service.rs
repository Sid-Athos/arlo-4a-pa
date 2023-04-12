use axum::http::StatusCode;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::user::User;

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
}