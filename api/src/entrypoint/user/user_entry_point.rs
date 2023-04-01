use axum::extract::{Path, State};
use axum::http::{HeaderMap, Request, StatusCode};
use axum::{Json, middleware, Router};
use axum::body::Body;
use axum::routing::{get, post};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::internal_error;
use crate::domain::model::session::Session;
use crate::domain::model::user::User;
use crate::entrypoint::middleware::is_logged::{is_logged};
use crate::entrypoint::user::request::create_user_request::CreateUserRequest;
use crate::entrypoint::user::request::login_request::LoginRequest;
use crate::service::command::create_user_command::CreateUserCommand;
use crate::service::command::login_command::LoginCommand;
use crate::service::session_service::SessionService;
use crate::service::user_service::UserService;

pub struct UserEntryPoint {}

impl UserEntryPoint {

    pub async fn user_get(State(pool): State<ConnectionPool>, Path(user_id): Path<i32>) -> Result<Json<User>, (StatusCode, String)> {
        let user_service = UserService::new(
            UserRepository::new(pool.clone()),
            SessionRepository::new(pool.clone())
        );

        let user = user_service.get_user_by_id(user_id).await?;

        Ok(Json(user))
    }

    pub async fn user_create(State(pool): State<ConnectionPool>, Json(user): Json<CreateUserRequest>) -> Result<Json<User>, (StatusCode, String)> {
        let user_service = UserService::new(
            UserRepository::new(pool.clone()),
            SessionRepository::new(pool.clone())
        );

        let user = user_service.create_user(CreateUserCommand::new(user)).await?;

        Ok(Json(user))
    }

    pub async fn user_login(State(pool): State<ConnectionPool>, Json(user): Json<LoginRequest>) -> Result<Json<Session>, (StatusCode, String)> {
        let user_service = UserService::new(
            UserRepository::new(pool.clone()),
            SessionRepository::new(pool.clone())
        );

        let session = user_service.login_user(LoginCommand::new(user)).await?;

        Ok(Json(session))
    }

    pub async fn user_logout(State(pool): State<ConnectionPool>, headers: HeaderMap) -> Result<Json<Session>, (StatusCode, String)> {
        let session_service = SessionService::new(
            UserRepository::new(pool.clone()),
            SessionRepository::new(pool.clone())
        );

        let token = headers["authorization"].to_str().map_err(internal_error)?.replace("Bearer ", "");
        let session = session_service.delete_token(token).await?;

        Ok(Json(session))
    }

    pub async fn me(State(pool): State<ConnectionPool>, headers: HeaderMap) -> Result<Json<User>, (StatusCode, String)> {
        let session_service = SessionService::new(
            UserRepository::new(pool.clone()),
            SessionRepository::new(pool.clone())
        );

        let token = headers["authorization"].to_str().map_err(internal_error)?.replace("Bearer ", "");
        let user = session_service.get_user_by_token(token).await?;

        Ok(Json(user))
    }

    pub fn get_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

        Router::new()
            .route("/create", post(UserEntryPoint::user_create))
            .route("/:user_id", get(UserEntryPoint::user_get))
            .route("/login", post(UserEntryPoint::user_login))
            .route("/logout", post(UserEntryPoint::user_logout))
            .route("/me", get(UserEntryPoint::me).route_layer(middleware::from_fn(is_logged)))
            .with_state(pool)
    }
}