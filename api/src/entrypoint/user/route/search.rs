use std::collections::HashMap;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::service::user_service::UserService;

#[utoipa::path(
    get,
    path = "/user/search?pseudo={pseudo}",
    responses(
        (status = 200, description = "Users found", body = Vec<UserResponse>),
    ),
    params(
        ("pseudo" = String,),
    ),
    tag = "user"
)]
pub async fn search(State(pool): State<ConnectionPool>, Query(params): Query<HashMap<String, String>>) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let pseudo = params.get("pseudo").ok_or_else(|| StatusCode::BAD_REQUEST)?.to_string();

    let users = user_service.search_user(pseudo).await?;

    Ok(Json(UserResponse::from_vec_domain(users)))
}