use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MessageRequest {
    pub to_user: i32,
    pub message: String,
}
