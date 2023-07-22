use crate::database::entity::game_members_history::GameMembersHistoryEntity;
use crate::domain::model::game_members_history::GameMembersHistory;

pub struct GameMembersHistoryEntityMapper;

impl GameMembersHistoryEntityMapper {

    pub fn entity_to_domain(game_members_history_entity: GameMembersHistoryEntity) -> GameMembersHistory {
        GameMembersHistory {
            id: game_members_history_entity.id,
            user_id: game_members_history_entity.user_id,
            game_history_id: game_members_history_entity.game_history_id,
            player: game_members_history_entity.player,
        }
    }
}