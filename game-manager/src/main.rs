mod entrypoint;

use std::net::SocketAddr;
use std::path::PathBuf;
use axum::Router;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
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


    let app = Router::new()
        .nest("/ws", ws_routes(connections));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

