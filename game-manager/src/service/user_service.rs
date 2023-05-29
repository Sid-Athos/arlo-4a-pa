use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::{database_error_to_response_error, database_error_to_status_code};
use crate::domain::model::user::User;

pub struct UserService {
    pub user_repository: UserRepository,
    pub session_repository: SessionRepository,
}

impl UserService {

    pub fn new(pool: ConnectionPool) -> Self {
        UserService {
            user_repository: UserRepository::new(pool.clone()),
            session_repository: SessionRepository::new(pool.clone()),
        }
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<User, StatusCode> {
        self.user_repository.get_user_by_id(user_id).await.map_err(database_error_to_status_code)
    }
}