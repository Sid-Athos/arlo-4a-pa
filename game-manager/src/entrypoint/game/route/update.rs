use axum::{Extension, Json};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::game::route::request::update_game_request::UpdateGameRequest;
use crate::entrypoint::game::route::response::game_response::GameResponse;
use crate::service::command::update_game_command::UpdateGameCommand;
use crate::service::game_service::GameService;

#[utoipa::path(
put,
path = "/game/{game_id}",
responses(
(status = 200, description = "game found", body = UserResponse),
(status = 401, description = "Invalid token",),
),
security(
("api-key" = []),
("bearer" = [])
),
request_body = UpdateUserRequest,
tag = "game"
)]
pub async fn update_game(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, Path(game_id): Path<i32>, Json(update_game_request): Json<UpdateGameRequest>) -> Result<Json<GameResponse>, StatusCode> {
    let game_service = GameService::new(
        pool.clone()
    );

    let command = UpdateGameCommand::new(game_id, update_game_request.name, update_game_request.description, update_game_request.language, update_game_request.min_players, update_game_request.max_players, update_game_request.code );

    let game = game_service.update_game(command, user.id).await?;

    Ok(Json(GameResponse::from_domain(game)))
}