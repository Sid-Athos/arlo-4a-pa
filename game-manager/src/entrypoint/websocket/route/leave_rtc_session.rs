use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::entrypoint::websocket::response::user_left_rtc_session::UserLeftRtcSessionResponse;
use crate::entrypoint::websocket::response::user_response::UserResponse;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;
use crate::service::ws_session_service::WsSessionService;

pub async fn leave_rtc_session(State(pool): State<ConnectionPool>, connections: Extension<Connections>, Extension(user): Extension<User>) -> Result<(), StatusCode> {

    let ws_service = WsSessionService::new(pool.clone());

    let member = ws_service.delete_for_user(user.id).await?;
    let members = ws_service.get_by_lobby_id(member.lobby_id).await?;

    let mut list_user_id = vec![];

    for member in members {
        list_user_id.push(member.user_id);
    }

    connections.send_to_vec_user_id(ResponseEnum::UserLeftRtcSession(UserLeftRtcSessionResponse::new(user.id)), list_user_id).await;

    Ok(())
}