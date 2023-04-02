use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::{Extension, Json, middleware, Router};
use axum::routing::{get, post};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::error::internal_error;
use crate::domain::model;
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

    async fn user_get(State(pool): State<ConnectionPool>, Path(user_id): Path<i32>) -> Result<Json<User>, (StatusCode, String)> {
        let user_service = UserService::new(
            UserRepository::new(pool.clone()),
            SessionRepository::new(pool.clone())
        );

        let user = user_service.get_user_by_id(user_id).await?;

        Ok(Json(user))
    }

    async fn user_create(State(pool): State<ConnectionPool>, Json(user): Json<CreateUserRequest>) -> Result<Json<User>, (StatusCode, String)> {
        let user_service = UserService::new(
            UserRepository::new(pool.clone()),
            SessionRepository::new(pool.clone())
        );

        let user = user_service.create_user(CreateUserCommand::new(user)).await?;

        Ok(Json(user))
    }

    async fn user_login(State(pool): State<ConnectionPool>, Json(user): Json<LoginRequest>) -> Result<Json<Session>, (StatusCode, String)> {
        let user_service = UserService::new(
            UserRepository::new(pool.clone()),
            SessionRepository::new(pool.clone())
        );

        let session = user_service.login_user(LoginCommand::new(user)).await?;

        Ok(Json(session))
    }

    async fn user_logout(State(pool): State<ConnectionPool>, Extension(session): Extension<Session>) -> Result<Json<Session>, (StatusCode, String)> {
        let session_service = SessionService::new(
            UserRepository::new(pool.clone()),
            SessionRepository::new(pool.clone())
        );

        let session = session_service.delete_token(session.token).await?;

        Ok(Json(session))
    }

    async fn me(Extension(user): Extension<User>) -> Result<Json<User>, (StatusCode, String)> {
        Ok(Json(user))
    }

    pub fn get_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

        Router::new()
            .route("/create", post(Self::user_create))
            .route("/:user_id", get(Self::user_get))
            .route("/login", post(Self::user_login))
            .route("/logout", post(Self::user_logout).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
            .route("/me", get(Self::me).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
            .with_state(pool)
    }
}