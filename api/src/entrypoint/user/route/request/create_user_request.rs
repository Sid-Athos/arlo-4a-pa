use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub nickname: String,
    pub email: String,
    pub password: String,
}