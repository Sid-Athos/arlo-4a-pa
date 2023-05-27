use axum::{
    extract::{Path, State},
    Json,
};
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::games_repository::GamesRepository;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::games::Game;
use crate::entrypoint::user::route::response::user_response::UserResponse;
use crate::service::games_service::GamesService;
use crate::service::user_service::UserService;


#[utoipa::path(
get,
path = "/games",
responses(
(status = 200, description = "Found games"),
(status = 401, description = "Unauthorized")
),
security(
("api-key" = []),
("bearer" = [])
),
tag="games"
)]
pub async fn get_available_games( State(pool): State<ConnectionPool> ) -> Result<Json<Vec<Game>>, StatusCode> {
    let games_service = GamesService::new(
        GamesRepository::new(pool.clone())
    );

    let games: Vec<Game> = games_service.get_all_games().await?;

    Ok(Json(games))
}