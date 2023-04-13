use serde::Serialize;
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
    UserJoined,
    UserKicked,
    UserInvited,
    InviteLobbyCancelled,
    InviteLobbyAccepted,
    InviteLobbyDeclined,
    HostGiven,
    StartedGame,
    BadMessage,
}
