use axum::{extract::{
    ws::{Message, WebSocket, WebSocketUpgrade},
    TypedHeader,
}, response::IntoResponse, routing::get, Router, Extension};
use std::borrow::Cow;
use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};
use std::collections::HashMap;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize};
use std::str;
use std::sync::Arc;
use axum::extract::State;
use chrono::{TimeZone, Utc};
use futures_util::stream::{SplitSink, SplitStream};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
struct Connections {
    clients: Arc<Mutex<Vec<SplitSink<WebSocket, Message>>>>,
}

impl Connections {
    fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn send_to_all(&self, message: Message) {
        let mut clients = self.clients.lock().await;
        println!("clients: {:?}", clients);
        let mut disconnected_clients = Vec::new();

        for (i, client) in clients.iter_mut().enumerate() {
            if client.send(message.clone()).await.is_err() {
                disconnected_clients.push(i);
            }
        }

        for i in disconnected_clients.into_iter().rev() {
            clients.remove(i);
        }
    }

    async fn add_client(&self, client: SplitSink<WebSocket, Message>) {
        self.clients.lock().await.push(client);
    }
}

#[tokio::main]
async fn main() {

    let connections = Connections::new();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route("/ws", get(ws_handler))
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)))
        .layer(Extension(connections.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    connections: Extension<Connections>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::info!("`{user_agent}` at {addr} connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, addr, connections))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, connections: Extension<Connections>) {
    let (mut sender, mut receiver) = socket.split();

    connections.add_client(sender).await;

    connections.send_to_all(Message::Text(format!("{} joined", who.to_string()))).await;

    while let Some(Ok(msg)) = receiver.next().await {
        connections.send_to_all(msg.clone()).await;
    }

    tracing::info!("Websocket context {} destroyed", who);
}
