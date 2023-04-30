use crate::database::entity::invite::InviteEntity;
use crate::domain::model::invite::Invite;

pub struct InviteEntityMapper;

impl InviteEntityMapper {

    pub fn entity_to_domain(invite_entity: InviteEntity) -> Invite {
        Invite {
            id: invite_entity.id,
            lobby_id: invite_entity.lobby_id,
            from_user_id: invite_entity.from_user_id,
            to_user_id: invite_entity.to_user_id,
        }
    }
}