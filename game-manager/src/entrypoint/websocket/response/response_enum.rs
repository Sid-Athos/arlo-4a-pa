use serde::Serialize;
use crate::entrypoint::websocket::response::emote_response::EmoteResponse;
use crate::entrypoint::websocket::response::game_actions_response::GameActionsResponse;
use crate::entrypoint::websocket::response::game_display_response::GameDisplayResponse;
use crate::entrypoint::websocket::response::game_started_response::GameStartedResponse;
use crate::entrypoint::websocket::response::ice_candidate_response::ICECandidateResponse;
use crate::entrypoint::websocket::response::invite_response::InviteResponse;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::message_response::MessageResponse;
use crate::entrypoint::websocket::response::sdp_answer_response::SDPAnswerResponse;
use crate::entrypoint::websocket::response::sdp_offer_response::SDPOfferResponse;
use crate::entrypoint::websocket::response::user_left_rtc_session::UserLeftRtcSessionResponse;

#[derive(Serialize, Debug)]
pub enum ResponseEnum {
    Hello,
    Pong,
    Bye,
    Message(MessageResponse),
    Emote(EmoteResponse),
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
    CannotStartGame,
    BadMessage,
    GameStopped,
    GameWin,
    GameLose,
    GameDisplay(GameDisplayResponse),
    GameAction(GameActionsResponse),
    GameStarted(GameStartedResponse),
    ICECandidate(ICECandidateResponse),
    SDPAnswer(SDPAnswerResponse),
    SDPOffer(SDPOfferResponse),
    Lobby(LobbyResponse),
    UserLeftRtcSession(UserLeftRtcSessionResponse),
    Error(String),
}
