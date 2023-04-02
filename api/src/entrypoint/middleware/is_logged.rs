use axum::{middleware:: Next, response::Response, http::Request, http};
use axum::extract::State;
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::session::Session;
use crate::service::session_service::SessionService;

pub async fn is_logged<B>(State(pool): State<ConnectionPool>, mut req: Request<B>, next: Next<B>) -> Result<Response, (StatusCode, String)> {
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err((StatusCode::UNAUTHORIZED, "No authorization header".to_string()));
    };
    let auth_header = auth_header.trim_start_matches("Bearer ");

    let session_service = SessionService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let user = session_service.get_user_by_token(auth_header.to_string()).await?;

    let session = Session {
        token: auth_header.to_string(),
        user_id: user.id,
        id: 0,
    };

    req.extensions_mut().insert(session);
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
