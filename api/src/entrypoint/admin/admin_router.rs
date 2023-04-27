use axum::{middleware, Router};
use axum::routing::{get, delete, put};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::admin::route::delete::delete_user;
use crate::entrypoint::admin::route::get_all::get_all;
use crate::entrypoint::admin::route::give_admin_role::give_admin_role;
use crate::entrypoint::admin::route::remove_admin_role::remove_admin_role;
use crate::entrypoint::admin::route::update_user::update_user;
use crate::entrypoint::middleware::is_logged_admin::is_logged_admin;
use crate::middlewares::cors_layer::init_cors_layer;

pub fn admin_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/get_all", get(get_all).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/delete/:user_id", delete(delete_user).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/give_admin_role/:user_id", put(give_admin_role).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/remove_admin_role/:user_id", put(remove_admin_role).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/update/:user_id", put(update_user).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .with_state(pool)

}