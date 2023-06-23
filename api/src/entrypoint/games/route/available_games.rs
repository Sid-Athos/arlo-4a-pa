use axum::{
    extract::{State},
    Json,
};
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::games_repository::GamesRepository;


use crate::domain::model::games::Game;

use crate::service::games_service::GamesService;



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
    println!("tamère3");
    let games: Vec<Game> = games_service.get_all_games().await?;

    Ok(Json(games))
}