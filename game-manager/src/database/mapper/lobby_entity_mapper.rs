use crate::database::entity::lobby::LobbyEntity;
use crate::domain::model::lobby::Lobby;

pub struct LobbyEntityMapper;

impl LobbyEntityMapper {

    pub fn entity_to_domain(lobby_entity: LobbyEntity) -> Lobby {
        Lobby {
            id: lobby_entity.id,
            game_id: lobby_entity.game_id,
            code: lobby_entity.code,
            private: lobby_entity.private,
            is_launched: lobby_entity.is_launched
        }
    }
}