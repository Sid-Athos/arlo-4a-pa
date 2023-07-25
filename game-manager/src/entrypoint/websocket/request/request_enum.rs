use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::LogError;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::request::accept_invite_lobby_request::AcceptInviteLobbyRequest;
use crate::entrypoint::websocket::request::cancel_invite_user_lobby_request::CancelInviteUserLobbyRequest;
use crate::entrypoint::websocket::request::create_lobby_request::CreateLobbyRequest;
use crate::entrypoint::websocket::request::create_lobby_by_game_move_history_id_request::CreateLobbyByGameMoveHistoryIdRequest;
use crate::entrypoint::websocket::request::decline_invite_lobby_request::DeclineInviteLobbyRequest;
use crate::entrypoint::websocket::request::emote_request::EmoteRequest;
use crate::entrypoint::websocket::request::exit_lobby_request::ExitLobbyRequest;
use crate::entrypoint::websocket::request::game_action_request::GameActionRequest;
use crate::entrypoint::websocket::request::give_lobby_host_request::GiveHostRequest;
use crate::entrypoint::websocket::request::invite_user_lobby_request::InviteUserLobbyRequest;
use crate::entrypoint::websocket::request::join_lobby_request::JoinLobbyRequest;
use crate::entrypoint::websocket::request::kick_user_lobby_request::KickUserRequest;
use crate::entrypoint::websocket::request::launch_game_request::LaunchGameRequest;
use crate::entrypoint::websocket::request::message_request::MessageRequest;
use crate::entrypoint::websocket::request::register_ice_candidate_request::RegisterICECandidateRequest;
use crate::entrypoint::websocket::request::sdp_answer_request::SDPAnswerRequest;
use crate::entrypoint::websocket::request::sdp_offer_request::SDPOfferRequest;
use crate::entrypoint::websocket::response::error_response::ErrorResponse;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;

use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::command::create_lobby_command_with_game_history_move_id::CreateLobbyCommandWithGameHistoryMoveId;

#[derive(Deserialize, Debug)]
pub enum RequestEnum {
    Hello,
    Ping,
    Exit,
    Message(MessageRequest),
    Emote(EmoteRequest),
    CreateLobby(CreateLobbyRequest),
    CreateLobbyByGameMoveHistoryIdRequest(CreateLobbyByGameMoveHistoryIdRequest),
    JoinLobby(JoinLobbyRequest),
    ExitLobby,
    GameAction(GameActionRequest),
    GiveHost(GiveHostRequest),
    KickUser(KickUserRequest),
    InviteUserLobby(InviteUserLobbyRequest),
    CancelInviteUserLobby(CancelInviteUserLobbyRequest),
    AcceptInviteLobby(AcceptInviteLobbyRequest),
    DeclineInviteLobby(DeclineInviteLobbyRequest),
    SDPOffer(SDPOfferRequest),
    SDPAnswer(SDPAnswerRequest),
    RegisterICECandidate(RegisterICECandidateRequest),
    LaunchGame,
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
            RequestEnum::GameAction(action) => {
                let action = action.compute(pool, connections.clone(), user.clone()).await;
                ErrorResponse::send_error(action, connections.clone(), user).await;
            }
            RequestEnum::Message(message) => {
                let response = message.compute(pool, connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user).await;
            }
            RequestEnum::Emote(message) => {
                let response = message.compute(pool, connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user).await;
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
            RequestEnum::AcceptInviteLobby(accept_invite_lobby_request) => {
                let response = accept_invite_lobby_request.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
                LobbyResponse::send_lobby_to_members(pool, connections.clone(), user.clone()).await.log_error();
            }
            RequestEnum::LaunchGame => {
                let response = LaunchGameRequest::compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
            }
            RequestEnum::SDPOffer(sdp_offer) => {
                let response = sdp_offer.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
            }
            RequestEnum::SDPAnswer(sdp_answer) => {
                let response = sdp_answer.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
            }
            RequestEnum::RegisterICECandidate(register_ice_candidate) => {
                let response = register_ice_candidate.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
            }
            RequestEnum::CreateLobbyByGameMoveHistoryIdRequest(create_lobby_request) => {
                let response = create_lobby_request.compute(pool.clone(), connections.clone(), user.clone()).await;
                ErrorResponse::send_error(response, connections.clone(), user.clone()).await;
                LobbyResponse::send_lobby_to_members(pool, connections.clone(), user.clone()).await.log_error();
            }
        }
        return false;
    }

}
