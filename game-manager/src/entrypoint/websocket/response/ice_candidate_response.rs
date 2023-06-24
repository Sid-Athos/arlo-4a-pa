use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ICECandidateResponse {
    candidate: String,
    sdp_mid: String,
    sdp_m_line_index: i32,
}

impl ICECandidateResponse {

    pub fn new(candidate: String, sdp_mid: String, sdp_m_line_index: i32) -> Self {
        ICECandidateResponse {
            candidate,
            sdp_mid,
            sdp_m_line_index,
        }
    }
}