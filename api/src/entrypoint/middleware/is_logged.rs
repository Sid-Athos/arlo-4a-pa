use axum::{middleware:: Next, response::Response, http::Request, http};
use axum::http::StatusCode;

pub async fn is_logged<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    Ok(next.run(req).await)
}
