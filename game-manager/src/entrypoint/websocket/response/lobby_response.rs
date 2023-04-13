use serde::Serialize;
use crate::entrypoint::websocket::response::lobby_member_response::LobbyMemberResponse;

#[derive(Serialize, Debug)]
pub struct LobbyResponse {
    pub id: i32,
    pub code: String,
    pub game_id: i32,
    pub private: bool,
    pub members: Vec<LobbyMemberResponse>,
}