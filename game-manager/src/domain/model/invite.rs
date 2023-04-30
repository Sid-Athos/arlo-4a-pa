use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Invite {
    pub id: i32,
    pub lobby_id: i32,
    pub from_user_id: i32,
    pub to_user_id: i32,
}
