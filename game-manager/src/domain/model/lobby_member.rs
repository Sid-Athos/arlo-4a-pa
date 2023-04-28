use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LobbyMember {
    pub id: i32,
    pub lobby_id: i32,
    pub user_id: i32,
    pub is_host: bool,
}
