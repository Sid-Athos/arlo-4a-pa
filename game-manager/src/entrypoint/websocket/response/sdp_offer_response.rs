use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SDPOfferResponse {
    sdp: String,
}

impl SDPOfferResponse {

    pub fn new(sdp: String) -> Self {
        SDPOfferResponse {
            sdp,
        }
    }
}