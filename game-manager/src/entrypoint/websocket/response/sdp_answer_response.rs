use serde::Serialize;
use crate::entrypoint::websocket::response::user_response::UserResponse;

#[derive(Serialize, Debug)]
pub struct SDPAnswerResponse {
    sdp: String,
    from_user: UserResponse,
}

impl SDPAnswerResponse {

    pub fn new(sdp: String, from_user: UserResponse) -> Self {
        SDPAnswerResponse {
            sdp,
            from_user
        }
    }
}