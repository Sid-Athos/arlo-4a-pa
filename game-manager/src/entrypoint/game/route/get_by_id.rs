use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::game::route::response::game_response::GameResponse;
use crate::service::game_service::GameService;

#[utoipa::path(
get,
path = "/game/{game_id}",
params(
("user_id" = String,),
),
responses(
(status = 200, description = "User found", body = UserResponse,),
(status = 404, description = "User not found",),
),
tag = "game"
)]
pub async fn get_by_id(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, Path(game_id): Path<i32>) -> Result<Json<GameResponse>, StatusCode> {
    let game_service = GameService::new(
        pool.clone()
    );

    let game = game_service.get_game_by_id(game_id, user.id).await.unwrap();

    Ok(Json(GameResponse::from_domain(game)))
}