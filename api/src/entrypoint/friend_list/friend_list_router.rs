use axum::{middleware, Router};
use axum::routing::{get, post, delete, put};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::friend_list::route::accept_friend_request::accept_friend_request;
use crate::entrypoint::friend_list::route::create::friend_list_create;
use crate::entrypoint::friend_list::route::delete_friend::delete_friend;
use crate::entrypoint::friend_list::route::show_friend_list::show_friend_list;
use crate::entrypoint::friend_list::route::show_friend_request::show_friend_request;


use crate::entrypoint::middleware::is_logged::{is_logged};
use crate::middlewares::swagger_security::check_api_key;

pub fn friend_list_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/create", post(friend_list_create).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged))).route_layer(middleware::from_fn(check_api_key))
        .route("/:friend_list_id", delete(delete_friend).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged))).route_layer(middleware::from_fn(check_api_key))
        .route("/:friend_list_id", put(accept_friend_request).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged))).route_layer(middleware::from_fn(check_api_key))
        .route("/", get(show_friend_list).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged))).route_layer(middleware::from_fn(check_api_key))
        .route("/requests", get(show_friend_request).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged))).route_layer(middleware::from_fn(check_api_key))
        .with_state(pool)

}