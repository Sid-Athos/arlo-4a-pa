use axum::{middleware, Router};
use axum::routing::get;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::admin::route::get_all::get_all;
use crate::entrypoint::middleware::is_logged_admin::is_logged_admin;

pub fn admin_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/get_all", get(get_all).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .with_state(pool)

}