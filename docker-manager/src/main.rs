mod entrypoint;
mod domain;
mod database;
mod service;

use std::env;
use std::net::SocketAddr;
use crate::database::init::init_db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracer();
    let pool = init_db().await.unwrap();

    let cors = init_cors_layer();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors);

    let addr: SocketAddr = (&env::var("SERVER").unwrap()).parse().expect("Not a socket address");

    tracing::info!("listening on {:?}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}