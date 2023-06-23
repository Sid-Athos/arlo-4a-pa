use axum::{middleware, Router};
use axum::routing::{get};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::middleware::is_logged::{is_logged};


use crate::entrypoint::games::route::available_games::get_available_games;
use crate::middlewares::swagger_security::check_api_key;

pub fn games_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {
    Router::new()
        .route("/games", get(get_available_games))
        .layer(middleware::from_fn(check_api_key))
        //.layer(middleware::from_fn_with_state(pool.clone(),is_logged))
        .with_state(pool)

}