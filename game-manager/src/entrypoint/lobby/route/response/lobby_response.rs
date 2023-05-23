use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::model::lobby::Lobby;
use crate::entrypoint::lobby::route::response::lobby_member_response::LobbyMemberResponse;

#[derive(Serialize, Debug)]
pub struct LobbyResponse {
    pub id: i32,
    pub code: String,
    pub game_id: i32,
    pub private: bool,
    pub members: Vec<LobbyMemberResponse>,
}

impl LobbyResponse {

    pub fn from_domain(lobby: Lobby, lobby_members: Vec<LobbyMemberResponse>) -> Self {
        LobbyResponse {
            id: lobby.id,
            code: lobby.code,
            game_id: lobby.game_id,
            private: lobby.private,
            members: lobby_members,
        }
    }
}