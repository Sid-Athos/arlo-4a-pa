use std::net::SocketAddr;
use axum::Extension;
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use colored::Colorize;
use futures_util::StreamExt;
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::request::request_enum::RequestEnum;

pub async fn ws_handler(ws: WebSocketUpgrade, ConnectInfo(_addr): ConnectInfo<SocketAddr>, State(pool): State<ConnectionPool>, connections: Extension<Connections>, Extension(user): Extension<User>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(pool, socket, connections, user))
}

async fn handle_socket(pool: ConnectionPool, socket: WebSocket, connections: Extension<Connections>, user: User) {
    let (sender, mut receiver) = socket.split();

    connections.add_client(user.id, sender).await;

    println!("{} : User {} connected", "INFO".blue(), user.id);

    while let Some(Ok(msg)) = receiver.next().await {
        let message: RequestEnum = match serde_json::from_str(msg.to_text().unwrap()) {
            Ok(message) => message,
            Err(_) => RequestEnum::BadMessage,
        };
        if message.compute(pool.clone(), connections.clone(), user.clone()).await {
            break
        }
    }

    exit_socket(pool, connections, user.clone()).await;

    println!("{} : User {} disconnected", "INFO".blue(), user.id);
}

async fn exit_socket(pool: ConnectionPool, connections: Extension<Connections>, user: User) {

    RequestEnum::ExitLobby.compute(pool, connections.clone(), user.clone()).await;
    connections.disconnect_client(user.id).await;
}
