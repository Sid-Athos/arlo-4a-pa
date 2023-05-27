use axum::{middleware, Router};
use axum::routing::{get, put, post, delete};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::game::route::create::create_game;
use crate::entrypoint::game::route::get_all_games::get_all_games;
use crate::entrypoint::middleware::is_logged::is_logged;

pub fn game_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/", get(get_all_games))
        .route("/", post(create_game).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .with_state(pool)

}
