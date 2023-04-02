use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json, middleware, Router};
use axum::routing::{get, post};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::session::Session;
use crate::domain::model::user::User;
use crate::entrypoint::middleware::is_logged::{is_logged};
use crate::entrypoint::user::request::create_user_request::CreateUserRequest;
use crate::entrypoint::user::request::login_request::LoginRequest;
use crate::service::command::create_user_command::CreateUserCommand;
use crate::service::command::login_command::LoginCommand;
use crate::service::session_service::SessionService;
use crate::service::user_service::UserService;

#[utoipa::path(
    get,
    path = "/user/{user_id}",
    params(
        ("user_id" = String,),
    ),
    responses(
        (status = 200, description = "User found", body = User,),
        (status = 404, description = "User not found",),
    )
)]
async fn user_get(State(pool): State<ConnectionPool>, Path(user_id): Path<i32>) -> Result<Json<User>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let user = user_service.get_user_by_id(user_id).await?;

    Ok(Json(user))
}

#[utoipa::path(
    post,
    path = "/user/create",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created", body = User,),
        (status = 409, description = "User email already exist",),
    )
)]
async fn user_create(State(pool): State<ConnectionPool>, Json(user): Json<CreateUserRequest>) -> Result<Json<User>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let user = user_service.create_user(CreateUserCommand::new(user)).await?;

    Ok(Json(user))
}

#[utoipa::path(
    post,
    path = "/user/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User connected", body = Session,),
        (status = 401, description = "Invalid password",),
        (status = 404, description = "User not found",),
    )
)]
async fn user_login(State(pool): State<ConnectionPool>, Json(user): Json<LoginRequest>) -> Result<Json<Session>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let session = user_service.login_user(LoginCommand::new(user)).await?;

    Ok(Json(session))
}

#[utoipa::path(
    post,
    path = "/user/logout",
    responses(
        (status = 200, description = "User disconnected", body = Session),
        (status = 401, description = "Invalid token",),
    ),
    security(
        ("BearerAuth" = ["read:items", "edit:items"])
    )
)]
async fn user_logout(State(pool): State<ConnectionPool>, Extension(session): Extension<Session>) -> Result<Json<Session>, StatusCode> {
    let session_service = SessionService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let session = session_service.delete_token(session.token).await?;

    Ok(Json(session))
}

#[utoipa::path(
    get,
    path = "/user/me",
    responses(
        (status = 200, description = "User found", body = User),
        (status = 401, description = "Invalid token",),
    ),
    security(
        ("BearerAuth" = ["read:items", "edit:items"])
    )
)]
async fn me(Extension(user): Extension<User>) -> Result<Json<User>, StatusCode> {
    Ok(Json(user))
}

#[utoipa::path(
    get,
    path = "/user/search?pseudo={pseudo}",
    responses(
        (status = 200, description = "Users found", body = Vec<User>),
    ),
    params(
        ("pseudo" = String,),
    )
)]
async fn search(State(pool): State<ConnectionPool>, Query(params): Query<HashMap<String, String>>) -> Result<Json<Vec<User>>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let pseudo = params.get("pseudo").ok_or_else(|| StatusCode::BAD_REQUEST)?.to_string();

    let users = user_service.search_user(pseudo).await?;

    Ok(Json(users))
}

pub fn get_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/:user_id", get(user_get))
        .route("/create", post(user_create))
        .route("/login", post(user_login))
        .route("/logout", post(user_logout).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/me", get(me).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/search", get(search))
        .with_state(pool)

}
