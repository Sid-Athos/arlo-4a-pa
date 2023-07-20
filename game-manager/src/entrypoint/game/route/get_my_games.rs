use axum::extract::{Path,State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::entrypoint::game::route::response::game_response::GameResponse;
use crate::service::game_service::GameService;

pub async fn get_my_games(State(pool): State<ConnectionPool>, Path(game_id): Path<i32>) -> Result<Json<Vec<GameResponse>>, StatusCode> {

    let game_service = GameService::new(pool);

    let games = game_service.get_my_games(game_id).await?;

    Ok(Json(GameResponse::from_vec_domain(games)))
}