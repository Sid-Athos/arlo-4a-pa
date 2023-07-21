use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json};
use crate::database::init::ConnectionPool;

use crate::entrypoint::game::route::response::game_response::GameResponse;
use crate::service::game_service::GameService;

#[utoipa::path(
get,
path = "/game/admin/{game_id}",
params(
("user_id" = String,),
),
responses(
(status = 200, description = "User found", body = UserResponse,),
(status = 404, description = "User not found",),
),
tag = "game"
)]
pub async fn delete_by_admin(State(pool): State<ConnectionPool>, Path(game_id): Path<i32>) -> Result<Json<GameResponse>, StatusCode> {
    let game_service = GameService::new(
        pool.clone()
    );

    let game = game_service.delete_by_admin(game_id).await?;

    Ok(Json(GameResponse::from_domain(game)))
}