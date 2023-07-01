use std::net::SocketAddr;
use axum::{Extension, middleware, Router};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::extract::ws::{WebSocket};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use colored::Colorize;
use futures_util::StreamExt;
use tokio_postgres::NoTls;
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::middleware::is_logged::is_logged;
use crate::entrypoint::middleware::is_logged_url::is_logged_url;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::request::request_enum::RequestEnum;
use crate::entrypoint::websocket::route::get_connected_friends::get_connected_friends;
use crate::entrypoint::websocket::route::join_rtc_session::join_rtc_session;
use crate::entrypoint::websocket::route::leave_rtc_session::leave_rtc_session;
use crate::entrypoint::websocket::route::ws_handler::ws_handler;

pub fn ws_routes(connections: Connections, pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/ws", get(ws_handler).layer(middleware::from_fn_with_state(pool.clone(), is_logged_url)))
        .route("/friends/connected_friends", get(get_connected_friends).layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/rtc/join_rtc", post(join_rtc_session).layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .route("/rtc/leave_rtc", post(leave_rtc_session).layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .layer(Extension(connections.clone()))
        .with_state(pool)
}
