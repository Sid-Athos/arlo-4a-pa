use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::request::accepte_invite_lobby_request::AcceptInviteLobbyRequest;
use crate::entrypoint::websocket::request::cancel_invite_user_lobby_request::CancelInviteUserLobbyRequest;
use crate::entrypoint::websocket::request::create_lobby_request::CreateLobbyRequest;
use crate::entrypoint::websocket::request::decline_invite_lobby_request::DeclineInviteLobbyRequest;
use crate::entrypoint::websocket::request::give_lobby_host_request::GiveHostRequest;
use crate::entrypoint::websocket::request::invite_user_lobby_request::InviteUserLobbyRequest;
use crate::entrypoint::websocket::request::join_lobby_request::JoinLobbyRequest;
use crate::entrypoint::websocket::request::kick_user_lobby_request::KickUserRequest;
use crate::entrypoint::websocket::request::message_request::MessageRequest;
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
                false
            }
            RequestEnum::Ping => {
                connections.send_to_vec_user_id(ResponseEnum::Pong, vec![user.id]).await;
                false
            }
            RequestEnum::Exit => {
                connections.send_to_vec_user_id(ResponseEnum::Bye, vec![user.id]).await;
                true
            }
            RequestEnum::Message(message) => {
                let response = MessageResponse {
                    from_user: user.id,
                    message: message.message.clone(),
                };
                connections.send_to_vec_user_id(ResponseEnum::Message(response), vec![message.to_user]).await;
                false
            }
            RequestEnum::BadMessage => {
                connections.send_to_vec_user_id(ResponseEnum::BadMessage, vec![user.id]).await;
                false
            }
            RequestEnum::CreateLobby(create_lobby_request) => {
                create_lobby_request.compute(pool, connections, user).await;
                false
            }
            _ => false,
        }
    }
}
