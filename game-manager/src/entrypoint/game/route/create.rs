use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;

use crate::domain::model::user::User;
use crate::entrypoint::game::route::request::create_game_request::CreateGameRequest;
use crate::entrypoint::game::route::response::game_response::GameResponse;
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
pub async fn create_game(State(pool): State<ConnectionPool>,  Extension(user): Extension<User>, Json(game): Json<CreateGameRequest>) -> Result<Json<GameResponse>, StatusCode> {
    let game_service = GameService::new(
        pool.clone()
    );
    //println!("{:?}",game);

    let game = game_service.create_game(game.name,game.max_players,game.min_players,game.description,game.language,game.code,user.id).await?;

    Ok(Json(GameResponse::from_domain(game)))
}