use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UserLeftRtcSessionResponse {
    pub user_id: int,
}

impl UserLeftRtcSessionResponse {
    pub fn new(user_id: int) -> Self {
        Self {
            user_id
        }
    }
}