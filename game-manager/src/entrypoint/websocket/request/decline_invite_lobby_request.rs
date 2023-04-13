use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeclineInviteLobbyRequest {
    lobby_id: i32
}