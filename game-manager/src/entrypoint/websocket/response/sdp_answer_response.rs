use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SDPAnswerResponse {
    sdp: String,
    from_user_id: i32
}

impl SDPAnswerResponse {

    pub fn new(sdp: String, from_user_id: i32) -> Self {
        SDPAnswerResponse {
            sdp,
            from_user_id
        }
    }
}