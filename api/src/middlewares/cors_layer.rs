use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

pub fn init_cors_layer()-> CorsLayer {
    CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
}