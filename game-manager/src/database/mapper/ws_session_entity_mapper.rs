use crate::database::entity::ws_session::WsSessionEntity;
use crate::domain::model::ws_session::WsSession;

pub struct WsSessionEntityMapper;

impl WsSessionEntityMapper {

    pub fn entity_to_domain(ws_session_entity: WsSessionEntity) -> WsSession {
        WsSession {
            id: ws_session_entity.id,
            lobby_id: ws_session_entity.lobby_id,
            user_id: ws_session_entity.user_id,
        }
    }
}