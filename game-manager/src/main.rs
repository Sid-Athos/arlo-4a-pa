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
use crate::entrypoint::game::game_router::game_routes;
use crate::entrypoint::lobby::lobby_router::lobby_routes;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::router::ws_routes;
use tower_http::cors::CorsLayer;

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
        .merge(lobby_routes(pool.clone()))
        .merge( game_routes(pool.clone()))
        .merge(ws_routes(connections, pool.clone()))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], 7589));

    println!("{} : listening on {}", "START".blue(), addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

