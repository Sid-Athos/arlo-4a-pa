use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LobbyMemberResponse {
    pub id: i32,
    pub pseudo: String,
    pub email: String,
    pub admin: bool,
    pub is_host: bool,
}
