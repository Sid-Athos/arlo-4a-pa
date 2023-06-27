use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::user_response::UserResponse;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;
use crate::service::ws_session_service::WsSessionService;

pub async fn join_rtc_session(State(pool): State<ConnectionPool>, connections: Extension<Connections>, Extension(user): Extension<User>) -> Result<Json<Vec<UserResponse>>, StatusCode> {

    let ws_service = WsSessionService::new(pool.clone());
    let user_service = UserService::new(pool.clone());
    let lobby_service = LobbyService::new(pool.clone());

    let lobby = lobby_service.get_by_user_id(user.id).await?;
    let members = ws_service.get_by_lobby_id(lobby.id).await?;

    let mut result = vec![];

    for member in members {
        result.push(UserResponse::from_domain(user_service.get_user_by_id(member.user_id).await?))
    }

    Ok(Json(result))
}