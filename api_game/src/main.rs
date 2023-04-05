mod domain;
mod database;
mod entrypoint;
mod service;

use axum::Router;
use std::net::SocketAddr;
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::database::init::init_db;
use crate::entrypoint::lobby::lobby_router::lobby_routes;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let pool = init_db().await.unwrap();

    let app = Router::new()
        .nest("/lobby", lobby_routes(pool.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
