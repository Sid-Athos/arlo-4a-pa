use serde::Serialize;
use crate::entrypoint::websocket::response::user_response::UserResponse;

#[derive(Serialize, Debug, Clone)]
pub struct ICECandidateResponse {
    candidate: String,
    sdp_mid: Option<String>,
    sdp_m_line_index: Option<i32>,
    from_user: UserResponse
}

impl ICECandidateResponse {

    pub fn new(candidate: String, sdp_mid: Option<String>, sdp_m_line_index: Option<i32>, from_user: UserResponse) -> Self {
        ICECandidateResponse {
            candidate,
            sdp_mid,
            sdp_m_line_index,
            from_user
        }
    }
}