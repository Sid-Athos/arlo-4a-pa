use axum::{middleware::Next, response::Response, http::Request};
use axum::extract::{State};
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::session::Session;
use crate::service::session_service::SessionService;

pub struct Token {
    token: String,
}

pub async fn is_logged_url<T>(State(pool): State<ConnectionPool>, mut req: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {
    let token = match req.uri().query() {
        Some(query) => query,
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    let split: Vec<&str> = token.split("&").collect();

    let mut token: Token = Token {
        token: String::default(),
    };

    for s in split {
        let token_str: Vec<&str> = s.split("=").collect();
        if token_str[0] == "token" {
            token.token = token_str[1].to_string();
            break;
        }
    }

    if token.token == String::default() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let session_service = SessionService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone())
    );

    let user = match session_service.get_user_by_token(token.token.clone()).await {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let session = Session {
        token: token.token,
        user_id: user.id,
        id: 0,
    };

    req.extensions_mut().insert(session);
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
