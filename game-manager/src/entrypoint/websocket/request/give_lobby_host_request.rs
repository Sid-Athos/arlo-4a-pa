use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GiveHostRequest {
    user_id: i32
}