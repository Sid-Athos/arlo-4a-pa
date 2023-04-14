use serde::Serialize;
use crate::domain::model::invite::Invite;

#[derive(Serialize, Debug)]
pub struct InviteResponse {
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub lobby_id: i32,
}

impl InviteResponse {
    pub fn from_domain(invite: Invite) -> Self {
        InviteResponse {
            from_user_id: invite.from_user_id,
            to_user_id: invite.to_user_id,
            lobby_id: invite.lobby_id,
        }
    }
}