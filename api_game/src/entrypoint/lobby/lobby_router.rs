use axum::{middleware, Router};
use axum::routing::{get, put, post, delete};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::lobby::route::create::create_lobby;
use crate::entrypoint::lobby::route::exit::exit;
use crate::entrypoint::lobby::route::get_member::get_lobby_member;
use crate::entrypoint::lobby::route::get_public::get_public_lobby;
use crate::entrypoint::lobby::route::get_public_for_game::get_public_lobby_for_game;
use crate::entrypoint::lobby::route::give_host::give_host;
use crate::entrypoint::lobby::route::join::join_lobby;
use crate::entrypoint::lobby::route::me::me;
use crate::entrypoint::lobby::route::search::search;
use crate::entrypoint::middleware::is_host::is_host;
use crate::entrypoint::middleware::is_logged::is_logged;

pub fn lobby_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/create", post(create_lobby).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/me", get(me).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/exit", delete(exit).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/:lobby_id", get(get_lobby_member))
        .route("/get_public", get(get_public_lobby))
        .route("/get_public/:game_id", get(get_public_lobby_for_game))
        .route("/search", get(search))
        .route("/join/:lobby_id", put(join_lobby).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/give_host/:user_id", put(give_host).route_layer(middleware::from_fn_with_state(pool.clone(), is_host)))
        .with_state(pool)

}
