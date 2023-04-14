use serde::Serialize;
use crate::entrypoint::websocket::response::invite_response::InviteResponse;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::message_response::MessageResponse;

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
    Lobby(LobbyResponse),
    Error(String),
}
