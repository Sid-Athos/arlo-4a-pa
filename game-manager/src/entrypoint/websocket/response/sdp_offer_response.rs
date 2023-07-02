use serde::Serialize;
use crate::entrypoint::websocket::response::user_response::UserResponse;

#[derive(Serialize, Debug)]
pub struct SDPOfferResponse {
    sdp: String,
    form_user: UserResponse,
}

impl SDPOfferResponse {

    pub fn new(sdp: String, form_user: UserResponse) -> Self {
        SDPOfferResponse {
            sdp,
            form_user
        }
    }
}