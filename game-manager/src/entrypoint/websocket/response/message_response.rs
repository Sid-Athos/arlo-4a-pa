use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MessageResponse {
    pub from_user: i32,
    pub message: String,
}