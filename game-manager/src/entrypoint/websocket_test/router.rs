use std::net::SocketAddr;
use axum::{Extension, middleware, Router};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::extract::ws::{WebSocket};
use axum::response::IntoResponse;
use axum::routing::get;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use colored::Colorize;
use futures_util::StreamExt;
use tokio_postgres::NoTls;
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::websocket_test::connections::ConnectionsTest;

pub fn ws_test_routes(connections: ConnectionsTest, pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/", get(ws_handler))
        .layer(Extension(connections.clone()))
        .with_state(pool)
}

async fn ws_handler(ws: WebSocketUpgrade, ConnectInfo(addr): ConnectInfo<SocketAddr>, State(pool): State<ConnectionPool>, connections: Extension<ConnectionsTest>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(pool, socket, connections))
}

async fn handle_socket(pool: ConnectionPool, socket: WebSocket, connections: Extension<ConnectionsTest>) {
    let (sender, mut receiver) = socket.split();

    println!("Connected");
    connections.add_client(sender).await;

    while let Some(Ok(msg)) = receiver.next().await {
        println!("Received message: {}", msg.to_text().unwrap());
        connections.send_to_all(msg).await;
    }
}
