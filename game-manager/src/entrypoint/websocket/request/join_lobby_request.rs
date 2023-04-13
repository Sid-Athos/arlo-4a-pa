use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JoinLobbyRequest {
    lobby_id: i32
}