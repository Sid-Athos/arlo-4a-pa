use serde::Serialize;
use crate::entrypoint::websocket::response::game_started_response::GameStartedResponse;
use crate::entrypoint::websocket::response::ice_candidate_response::ICECandidateResponse;
use crate::entrypoint::websocket::response::invite_response::InviteResponse;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::message_response::MessageResponse;
use crate::entrypoint::websocket::response::sdp_answer_response::SDPAnswerResponse;
use crate::entrypoint::websocket::response::sdp_offer_response::SDPOfferResponse;

#[derive(Serialize, Debug)]
pub enum ResponseEnum {
    Hello,
    Pong,
    Bye,
    Message(MessageResponse),
    LobbyCreated,
    LobbyJoined,
    LobbyExited,
    LobbyHostGiven,
    LobbyHostTaken,
    UserJoined,
    UserKicked,
    Kicked,
    UserInvited,
    InviteLobbyCancelled(InviteResponse),
    InviteLobbyAccepted(InviteResponse),
    InviteLobbyDeclined(InviteResponse),
    InviteReceived(InviteResponse),
    StartedGame,
    BadMessage,
    GameStarted(GameStartedResponse),
    ICECandidate(ICECandidateResponse),
    SDPAnswer(SDPAnswerResponse),
    SDPOffer(SDPOfferResponse),
    Lobby(LobbyResponse),
    Error(String),
}
