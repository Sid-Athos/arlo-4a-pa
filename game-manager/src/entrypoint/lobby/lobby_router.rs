use axum::{Router};
use axum::routing::{get};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::lobby::route::get_public_lobby_for_game::get_public_lobby_for_game;

pub fn lobby_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/get_public/:game_id", get(get_public_lobby_for_game))
        .with_state(pool)

}
