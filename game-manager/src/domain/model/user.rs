use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub pseudo: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
}
