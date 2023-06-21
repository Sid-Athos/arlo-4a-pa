use axum::http::StatusCode;

pub fn internal_error<E>(_: E) -> StatusCode where E: std::error::Error {
    StatusCode::INTERNAL_SERVER_ERROR
}
