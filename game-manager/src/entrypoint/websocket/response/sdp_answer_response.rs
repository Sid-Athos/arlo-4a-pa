use serde::Serialize;
use crate::entrypoint::websocket::response::user_response::UserResponse;

#[derive(Serialize, Debug)]
pub struct SDPAnswerResponse {
    sdp: String,
    form_user: UserResponse,
}

impl SDPAnswerResponse {

    pub fn new(sdp: String, form_user: UserResponse) -> Self {
        SDPAnswerResponse {
            sdp,
            form_user
        }
    }
}