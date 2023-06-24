use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SDPAnswerResponse {
    sdp: String,
}

impl SDPAnswerResponse {

    pub fn new(sdp: String) -> Self {
        SDPAnswerResponse {
            sdp,
        }
    }
}