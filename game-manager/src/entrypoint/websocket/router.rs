use std::net::SocketAddr;
use axum::{Extension, Router, TypedHeader};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::routing::get;
use futures_util::StreamExt;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use crate::entrypoint::websocket::connections::Connections;

pub fn ws_routes(connections: Connections) -> Router {

    Router::new()
        .route("/", get(ws_handler))
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)))
        .layer(Extension(connections.clone()))

}

async fn ws_handler(ws: WebSocketUpgrade, user_agent: Option<TypedHeader<headers::UserAgent>>, ConnectInfo(addr): ConnectInfo<SocketAddr>, connections: Extension<Connections>, ) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::info!("`{user_agent}` at {addr} connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, addr, connections))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr, connections: Extension<Connections>) {
    let (sender, mut receiver) = socket.split();

    connections.add_client(who.to_string().replace("127.0.0.1:", "").parse().unwrap(), sender).await;

    connections.send_to_all(Message::Text(format!("{} joined", who.to_string()))).await;

    while let Some(Ok(msg)) = receiver.next().await {
        connections.send_to_all(msg.clone()).await;
    }

    tracing::info!("Websocket context {} destroyed", who);
}
