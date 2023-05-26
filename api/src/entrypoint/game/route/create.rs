use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::entrypoint::game::route::request::create_game_request::CreateGameRequest;
use crate::entrypoint::game::route::response::game_response::GameResponse;
use crate::service::command::create_game_command::CreateGameCommand;
use crate::service::game_service::GameService;

#[utoipa::path(
post,
path = "/game/",
request_body = CreateGameRequest,
responses(
(status = 200, description = "Game created", body = GameResponse,),
(status = 409, description = "Game already exist",),
),
tag="game"
)]
pub async fn create_game(State(pool): State<ConnectionPool>, Json(game): Json<CreateGameRequest>) -> Result<Json<GameResponse>, StatusCode> {
    let game_service = GameService::new(
        GameRepository::new(pool.clone())
    );

    let game = game_service.create_game(CreateGameCommand::new(game)).await?;

    Ok(Json(GameResponse::from_domain(game)))
}