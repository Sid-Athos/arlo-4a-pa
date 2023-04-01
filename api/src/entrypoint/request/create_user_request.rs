use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub pseudo: String,
    pub email: String,
    pub password: String,
}