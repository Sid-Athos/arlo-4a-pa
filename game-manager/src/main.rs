mod entrypoint;
mod domain;
mod database;
mod service;

use std::net::SocketAddr;
use axum::Router;
use colored::Colorize;
use dotenv::dotenv;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::database::init::init_db;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::router::ws_routes;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let pool = init_db().await.unwrap();

    let connections = Connections::new();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let app = Router::new()
        .nest("/ws", ws_routes(connections, pool.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("{} : listening on {}", "START".blue(), addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

