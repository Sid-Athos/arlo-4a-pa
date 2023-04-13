use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CancelInviteUserLobbyRequest {
    user_id: i32
}