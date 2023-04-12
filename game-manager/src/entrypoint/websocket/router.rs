use std::net::SocketAddr;
use axum::{Extension, middleware, Router, TypedHeader};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::routing::get;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use colored::Colorize;
use futures_util::StreamExt;
use tokio_postgres::NoTls;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use crate::domain::model::user::User;
use crate::entrypoint::middleware::is_logged::is_logged;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::request::request_enum::RequestEnum;

pub fn ws_routes(connections: Connections, pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {

    Router::new()
        .route("/", get(ws_handler).route_layer(middleware::from_fn_with_state(pool.clone(), is_logged)))
        .layer(Extension(connections.clone()))

}

async fn ws_handler(ws: WebSocketUpgrade, ConnectInfo(addr): ConnectInfo<SocketAddr>, connections: Extension<Connections>, Extension(user): Extension<User>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr, connections, user))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr, connections: Extension<Connections>, user: User) {
    let (sender, mut receiver) = socket.split();

    let user_id = user.id;

    connections.add_client(user_id, sender).await;

    println!("{} : User {} connected", "INFO".blue(), user_id);

    while let Some(Ok(msg)) = receiver.next().await {
        let message: RequestEnum = match serde_json::from_str(msg.to_text().unwrap()) {
            Ok(message) => message,
            Err(_) => RequestEnum::BadMessage,
        };
        if message.compute(connections.clone(), user.clone()).await {
            break
        }
    }

    connections.disconnect_client(user_id).await;

    println!("{} : User {} disconnected", "INFO".blue(), user_id);
}
