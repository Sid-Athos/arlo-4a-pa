use axum::{middleware, Router};
use bb8::Pool;
use axum::routing::{get, delete, put, post};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::middleware::is_logged_admin::is_logged_admin;
use crate::entrypoint::ranking::route::init_ranking::init_ranking;
use crate::entrypoint::ranking::route::update_ranking::update_ranking;

pub fn ranking_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/init", post(init_ranking).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/", put(update_ranking).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .with_state(pool)

}