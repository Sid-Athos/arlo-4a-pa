use axum::{middleware, Router};
use axum::routing::{get, put, post, delete};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::game::route::get_all_games::get_all_games;

pub fn game_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/all", get(get_all_games))
        .with_state(pool)

}
