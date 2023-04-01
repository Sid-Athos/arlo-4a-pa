mod database;
mod service;
mod domain;
mod entrypoint;

use axum::Router;
use std::net::SocketAddr;
use dotenv::dotenv;
use crate::database::init::init_db;
use crate::entrypoint::user::UserEntryPoint;


#[tokio::main]
async fn main() {

    dotenv().ok();

    let pool = init_db().await.unwrap();

    let app = Router::new()
        .nest("/user", UserEntryPoint::get_routes(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
