use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InviteUserLobbyRequest {
    user_id: i32
}