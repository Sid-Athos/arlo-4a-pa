use serde::Serialize;
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
    InviteLobbyCancelled,
    InviteLobbyAccepted,
    InviteLobbyDeclined,
    StartedGame,
    BadMessage,
    Lobby(LobbyResponse),
    Error(String),
}
