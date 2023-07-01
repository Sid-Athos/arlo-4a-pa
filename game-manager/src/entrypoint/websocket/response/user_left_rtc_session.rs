use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UserLeftRtcSessionResponse {
    pub user_id: i32,
}

impl UserLeftRtcSessionResponse {
    pub fn new(user_id: i32) -> Self {
        Self {
            user_id
        }
    }
}