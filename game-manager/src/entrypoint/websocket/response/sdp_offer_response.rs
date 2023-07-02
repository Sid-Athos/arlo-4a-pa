use serde::Serialize;
use crate::entrypoint::websocket::response::user_response::UserResponse;

#[derive(Serialize, Debug)]
pub struct SDPOfferResponse {
    sdp: String,
    from_user: UserResponse,
}

impl SDPOfferResponse {

    pub fn new(sdp: String, from_user: UserResponse) -> Self {
        SDPOfferResponse {
            sdp,
            from_user
        }
    }
}