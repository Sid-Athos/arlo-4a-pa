use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SDPOfferResponse {
    sdp: String,
    from_user_id: i32,
}

impl SDPOfferResponse {

    pub fn new(sdp: String, from_user_id: i32) -> Self {
        SDPOfferResponse {
            sdp,
            from_user_id
        }
    }
}