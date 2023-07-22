use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::entrypoint::history::route::response::game_move_history_response::GameMoveHistoryResponse;
use crate::service::game_move_history_service::GameMoveHistoryService;

pub async fn get_game_move_history_by_game_history_id(State(pool): State<ConnectionPool>, Path(game_history_id): Path<i32>) -> Result<Json<Vec<GameMoveHistoryResponse>>, StatusCode> {
    let game_move_history_service = GameMoveHistoryService::new(pool.clone());
    let game_move_history_vec = game_move_history_service.get_all_by_game_history_id(game_history_id).await?;

    let mut result = Vec::new();

    for game_history in game_move_history_vec {
        result.push(GameMoveHistoryResponse::from_domain(game_history));
    }

    Ok(Json(result))
}