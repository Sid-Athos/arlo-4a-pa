use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::LogError;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::request::accepte_invite_lobby_request::AcceptInviteLobbyRequest;
use crate::entrypoint::websocket::request::cancel_invite_user_lobby_request::CancelInviteUserLobbyRequest;
use crate::entrypoint::websocket::request::create_lobby_request::CreateLobbyRequest;
use crate::entrypoint::websocket::request::decline_invite_lobby_request::DeclineInviteLobbyRequest;
use crate::entrypoint::websocket::request::exit_lobby_request::ExitLobbyRequest;
use crate::entrypoint::websocket::request::give_lobby_host_request::GiveHostRequest;
use crate::entrypoint::websocket::request::invite_user_lobby_request::InviteUserLobbyRequest;
use crate::entrypoint::websocket::request::join_lobby_request::JoinLobbyRequest;
use crate::entrypoint::websocket::request::kick_user_lobby_request::KickUserRequest;
use crate::entrypoint::websocket::request::message_request::MessageRequest;
use crate::entrypoint::websocket::response::error_response::ErrorResponse;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::message_response::MessageResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;

#[derive(Deserialize, Debug)]
pub enum RequestEnum {
    Hello,
    Ping,
    Exit,
    Message(MessageRequest),
    CreateLobby(CreateLobbyRequest),
    JoinLobby(JoinLobbyRequest),
    ExitLobby,
    GiveHost(GiveHostRequest),
    KickUser(KickUserRequest),
    InviteUserLobby(InviteUserLobbyRequest),
    CancelInviteUserLobby(CancelInviteUserLobbyRequest),
    AcceptInviteLobby(AcceptInviteLobbyRequest),
    DeclineInviteLobby(DeclineInviteLobbyRequest),
    BadMessage,
}

impl RequestEnum {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> bool {
        match self {
            RequestEnum::Hello => {
                connections.send_to_vec_user_id(ResponseEnum::Hello, vec![user.id]).await;
            }
            RequestEnum::Ping => {
                connections.send_to_vec_user_id(ResponseEnum::Pong, vec![user.id]).await;
            }
            RequestEnum::Exit => {
                connections.send_to_vec_user_id(ResponseEnum::Bye, vec![user.id]).await;
                return true;
            }
            RequestEnum::Message(message) => {
                let response = MessageResponse {
                    from_user: user.id,
                    message: message.message.clone(),
                };
                connections.send_to_vec_user_id(ResponseEnum::Message(response), vec![message.to_user]).await;
            }
            RequestEnum::BadMessage => {
                connections.send_to_vec_user_id(ResponseEnum::BadMessage, vec![user.id]).await;
            }
            RequestEnum::CreateLobby(create_lobby_request) => {
                let response = create_lobby_request.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
                LobbyResponse::send_lobby_to_members(pool, connections.clone(), user.clone()).await.log_error();
            }
            RequestEnum::JoinLobby(join_lobby_request) => {
                let response = join_lobby_request.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
                LobbyResponse::send_lobby_to_members(pool, connections.clone(), user.clone()).await.log_error();
            }
            RequestEnum::ExitLobby => {
                let response = ExitLobbyRequest::compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
                LobbyResponse::send_lobby_to_members(pool, connections.clone(), user.clone()).await.log_error();
            }
            RequestEnum::GiveHost(give_host_request) => {
                let response = give_host_request.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
                LobbyResponse::send_lobby_to_members(pool, connections.clone(), user.clone()).await.log_error();
            }
            RequestEnum::KickUser(kick_user_request) => {
                let response = kick_user_request.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
                LobbyResponse::send_lobby_to_members(pool, connections.clone(), user.clone()).await.log_error();
            }
            RequestEnum::InviteUserLobby(invite_user_lobby_request) => {
                let response = invite_user_lobby_request.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
            }
            RequestEnum::CancelInviteUserLobby(cancel_invite_user_lobby_request) => {
                let response = cancel_invite_user_lobby_request.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
            }
            RequestEnum::DeclineInviteLobby(decline_invite_lobby_request) => {
                let response = decline_invite_lobby_request.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
            }
            _ => {},
        }
        return false;
    }

}
