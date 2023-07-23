use axum::extract::{State};
use axum::http::StatusCode;
use axum::{Extension,Json};
use crate::database::init::ConnectionPool;
use crate::entrypoint::game::route::response::game_response::GameResponse;
use crate::service::game_service::GameService;
use crate::domain::model::user::User;

pub async fn get_my_games(State(pool): State<ConnectionPool>, Extension(user): Extension<User>) -> Result<Json<Vec<GameResponse>>, StatusCode> {

    let game_service = GameService::new(pool);

    let games = game_service.get_my_games(user.id).await?;

    Ok(Json(GameResponse::from_vec_domain(games)))
}