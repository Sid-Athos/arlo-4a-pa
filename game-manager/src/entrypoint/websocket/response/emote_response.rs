use serde::Serialize;
use crate::domain::model::user::User;

#[derive(Serialize, Debug)]
pub struct EmoteResponse {
    pub from_user: User,
    pub emote: String,
}