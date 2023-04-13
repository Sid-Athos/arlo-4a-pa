use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct KickUserRequest {
    user_id: i32
}