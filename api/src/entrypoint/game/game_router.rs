use axum::{middleware, Router};
use bb8::Pool;
use axum::routing::{get, post, delete, put};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::game::route::create::create_game;
use crate::entrypoint::middleware::is_logged::{is_logged};
use crate::middlewares::swagger_security::check_api_key;

pub fn game_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/", post(create_game))
        .layer(middleware::from_fn(check_api_key))
        .layer(middleware::from_fn_with_state(pool.clone(), is_logged))
        .with_state(pool)

}