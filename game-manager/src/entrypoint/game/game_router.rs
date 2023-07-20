use axum::{middleware, Router};
use axum::routing::{get, put, post, delete};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::game::route::create::create_game;
use crate::entrypoint::game::route::delete_by_admin::delete_by_admin;
use crate::entrypoint::game::route::delete_by_user::delete_by_user;
use crate::entrypoint::game::route::get_all_games::get_all_games;
use crate::entrypoint::game::route::get_by_id::get_by_id;
use crate::entrypoint::game::route::update::update_game;
use crate::entrypoint::middleware::is_logged::is_logged;
use crate::entrypoint::middleware::is_logged_admin::is_logged_admin;

pub fn game_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/games/all", get(get_all_games))
        .route("/games/mine", get(get_my_games)/**.route_layer(middleware::from_fn_with_state(pool.clone(), is_logged))*/)
        .route("/games/:game_id", get(get_by_id).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/games/admin/:game_id", delete(delete_by_admin).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged_admin)))
        .route("/games/:game_id", delete(delete_by_user).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/games/:game_id", put(update_game).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/games", post(create_game).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .with_state(pool)

}
