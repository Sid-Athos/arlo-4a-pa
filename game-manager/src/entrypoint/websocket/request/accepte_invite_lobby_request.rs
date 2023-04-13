use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AcceptInviteLobbyRequest {
    lobby_id: i32
}