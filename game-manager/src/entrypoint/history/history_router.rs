use axum::{middleware, Router};
use axum::routing::get;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::entrypoint::history::route::get_game_history::get_game_history_by_user_id_and_game_id;
use crate::entrypoint::history::route::get_game_move_history::get_game_move_history_by_game_history_id;
use crate::entrypoint::middleware::is_logged::is_logged;

pub fn history_routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/history/moves/:game_history_id", get(get_game_move_history_by_game_history_id).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/history/games/:game_id", get(get_game_history_by_user_id_and_game_id).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .with_state(pool)

}
