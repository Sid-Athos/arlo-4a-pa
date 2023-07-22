use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::history::route::response::game_history_response::GameHistoryResponse;
use crate::service::game_history_service::GameHistoryService;

pub async fn get_game_history_by_user_id_and_game_id(State(pool): State<ConnectionPool>, Path(game_id): Path<i32>,  Extension(user): Extension<User>) -> Result<Json<Vec<GameHistoryResponse>>, StatusCode> {
    let game_history_service = GameHistoryService::new(pool.clone());
    let game_history_vec = game_history_service.get_all_by_user_id_and_game_id(user.id, game_id).await?;

    let mut result = Vec::new();

    for game_history in game_history_vec {
        result.push(GameHistoryResponse::from_domain(game_history, pool.clone()).await?);
    }

    Ok(Json(result))
}