use serde::Serialize;
use crate::domain::model::lobby_member::LobbyMember;
use crate::domain::model::user::User;

#[derive(Serialize, Debug)]
pub struct LobbyMemberResponse {
    pub id: i32,
    pub pseudo: String,
    pub email: String,
    pub admin: bool,
    pub is_host: bool,
}

impl LobbyMemberResponse {
    pub fn from_domain(user: User, lobby_member: LobbyMember) -> Self {
        LobbyMemberResponse {
            id: user.id,
            pseudo: user.pseudo,
            email: user.email,
            admin: user.admin,
            is_host: lobby_member.is_host,
        }
    }
}
