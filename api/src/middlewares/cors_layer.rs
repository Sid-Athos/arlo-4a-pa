use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

pub fn init_cors_layer()-> CorsLayer {
    CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any)
        .expose_headers(Any)
}