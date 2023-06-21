mod entrypoint;
mod domain;
mod service;
mod middlewares;

use std::env;
use std::net::SocketAddr;
use axum::Router;
use dotenv::dotenv;
use crate::middlewares::{tracing::init_tracer, cors_layer::init_cors_layer};
use utoipa::{ OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use crate::entrypoint::docker_image::docker_image_router::docker_image_routes;
use crate::middlewares::swagger_security::SecurityAddon;
use crate::entrypoint::docker_image::route::request::create_image_request::CreateImageRequest;
use crate::entrypoint::docker_image::route::response::create_image_response::CreateImageResponse;


#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracer();
    let cors = init_cors_layer();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(docker_image_routes())
        .layer(cors);

    let addr: SocketAddr = (&env::var("SERVER").unwrap()).parse().expect("Not a socket address");

    tracing::info!("listening on {:?}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

#[derive(OpenApi)]
#[openapi(
paths(
entrypoint::docker_image::route::create::image_create
),
modifiers(&SecurityAddon),
components(
schemas(CreateImageRequest),
schemas(CreateImageResponse)
),
)]
struct ApiDoc;