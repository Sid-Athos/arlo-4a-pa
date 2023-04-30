use std::str::FromStr;
use axum::http::HeaderName;
use tower_http::cors::{AllowCredentials, AllowHeaders, AllowOrigin, Any, CorsLayer};


pub fn init_cors_layer()-> CorsLayer {
    let iter = vec![HeaderName::from_str("api-key").unwrap(), HeaderName::from_str("Content-Type").unwrap()];
    CorsLayer::new()
        .allow_headers(AllowHeaders::from(iter))
        .allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
}