use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ICECandidateResponse {
    candidate: String,
    sdp_mid: Option<String>,
    sdp_m_line_index: Option<i32>,
    from_user_id: i32
}

impl ICECandidateResponse {

    pub fn new(candidate: String, sdp_mid: Option<String>, sdp_m_line_index: Option<i32>, from_user_id: i32) -> Self {
        ICECandidateResponse {
            candidate,
            sdp_mid,
            sdp_m_line_index,
            from_user_id
        }
    }
}