use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::entrypoint::game::route::response::game_response::GameResponse;
use crate::service::game_service::GameService;

pub async fn get_all_games(State(pool): State<ConnectionPool>) -> Result<Json<Vec<GameResponse>>, StatusCode> {

    let game_service = GameService::new(pool);

    let games = game_service.get_all_games().await?;

    Ok(Json(GameResponse::from_vec_domain(games)))
}