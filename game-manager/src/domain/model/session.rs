use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
}