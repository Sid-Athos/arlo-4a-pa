use serde::Serialize;
use crate::domain::model::user::User;

#[derive(Serialize, Debug)]
pub struct MessageResponse {
    pub from_user: User,
    pub message: String,
}