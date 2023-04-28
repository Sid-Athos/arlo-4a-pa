use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use axum::http::Method;

pub fn init_cors_layer()-> CorsLayer {
    CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(AllowOrigin::exact("http://localhost:8080".parse().unwrap()))
        .expose_headers(Any)
}