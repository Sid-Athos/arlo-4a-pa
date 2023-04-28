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
    pub fn from_domain(friendList: FriendList) -> Self {
        FriendListResponse {
            id: friendList.id,
            applicant_id: friendList.applicant_id,
            recipient_id: friendList.recipient_id,
            accepted: friendList.accepted,
        }
    }

    pub fn from_vec_domain(friendLists: Vec<FriendList>) -> Vec<Self> {
        friendLists.into_iter().map(|friend| FriendListResponse::from_domain(friend)).collect()
    }
}