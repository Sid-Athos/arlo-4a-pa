use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{get, post};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::init::{ConnectionPool, DatabaseConnection};
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::internal_error;
use crate::domain::model::session::Session;
use crate::domain::model::user::User;
use crate::entrypoint::command::create_user_command::CreateUserCommand;
use crate::entrypoint::command::login_command::LoginCommand;
use crate::entrypoint::request::create_user_request::CreateUserRequest;
use crate::entrypoint::request::login_request::LoginRequest;
use crate::service::user_service::UserService;

pub struct UserEntryPoint {}

impl UserEntryPoint {

    pub async fn user_get(State(pool): State<ConnectionPool>, Path(user_id): Path<i32>) -> Result<Json<User>, (StatusCode, String)> {
        let user_service = UserService::new(UserRepository::new(pool.clone()), SessionRepository::new(pool.clone()));

        let user = user_service.get_user_by_id(user_id).await?;

        Ok(Json(user))
    }

    pub async fn user_create(State(pool): State<ConnectionPool>, Json(user): Json<CreateUserRequest>) -> Result<Json<User>, (StatusCode, String)> {
        let user_service = UserService::new(UserRepository::new(pool.clone()), SessionRepository::new(pool.clone()));

        let user = user_service.create_user(CreateUserCommand::new(user)).await?;

        Ok(Json(user))
    }

    pub async fn user_login(State(pool): State<ConnectionPool>, Json(user): Json<LoginRequest>) -> Result<Json<Session>, (StatusCode, String)> {
        let user_service = UserService::new(UserRepository::new(pool.clone()), SessionRepository::new(pool.clone()));

        let session = user_service.login_user(LoginCommand::new(user)).await?;

        Ok(Json(session))
    }

    pub fn get_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {
        Router::new()
            .route("/create", post(UserEntryPoint::user_create))
            .route("/:user_id", get(UserEntryPoint::user_get))
            .route("/login", post(UserEntryPoint::user_login))
            .with_state(pool)
    }
}