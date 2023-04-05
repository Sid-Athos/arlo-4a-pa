use axum::{middleware, Router};
use axum::routing::get;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::lobby::route::get_public::get_public_lobby;
use crate::entrypoint::middleware::is_logged::is_logged;

pub fn lobby_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/get_public", get(get_public_lobby))
        .with_state(pool)

}
