use axum::Router;
use axum::routing::post;
use crate::entrypoint::docker_image::route::create::image_create;

pub fn docker_image_routes() -> Router {
    Router::new()
        .route("/", post(image_create))
}