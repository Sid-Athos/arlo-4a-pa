mod entrypoint;

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
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::router::ws_routes;

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
        .nest("/ws", ws_routes(connections));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

