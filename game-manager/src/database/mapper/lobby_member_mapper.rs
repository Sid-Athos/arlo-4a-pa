use crate::database::entity::lobby_member::LobbyMemberEntity;
use crate::domain::model::lobby_member::LobbyMember;

pub struct LobbyMemberEntityMapper;

impl LobbyMemberEntityMapper {

    pub fn entity_to_domain(lobby_member_entity: LobbyMemberEntity) -> LobbyMember {
        LobbyMember {
            id: lobby_member_entity.id,
            lobby_id: lobby_member_entity.lobby_id,
            user_id: lobby_member_entity.user_id,
            is_host: lobby_member_entity.is_host,
        }
    }
}