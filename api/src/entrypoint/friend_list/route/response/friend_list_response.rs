use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::model::friend_list::FriendList;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct FriendListResponse {
    pub id: i32,
    pub applicant_id: i32,
    pub recipient_id: i32,
    pub accepted: bool,
}

impl FriendListResponse {
    pub fn from_domain(friend_list: FriendList) -> Self {
        FriendListResponse {
            id: friend_list.id,
            applicant_id: friend_list.applicant_id,
            recipient_id: friend_list.recipient_id,
            accepted: friend_list.accepted,
        }
    }

    pub fn from_vec_domain(friend_lists: Vec<FriendList>) -> Vec<Self> {
        friend_lists.into_iter().map(|friend| FriendListResponse::from_domain(friend)).collect()
    }
}