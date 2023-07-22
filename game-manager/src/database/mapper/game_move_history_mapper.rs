use crate::database::entity::game_move_history::GameMoveHistoryEntity;
use crate::domain::model::game_move_history::GameMoveHistory;

pub struct GameMoveHistoryEntityMapper;

impl GameMoveHistoryEntityMapper {

    pub fn entity_to_domain(game_move_history_entity: GameMoveHistoryEntity) -> GameMoveHistory {
        GameMoveHistory {
            id: game_move_history_entity.id,
            player: game_move_history_entity.player,
            game_state: game_move_history_entity.game_state,
            action: game_move_history_entity.action,
            action_number: game_move_history_entity.action_number,
            game_history_id: game_move_history_entity.game_history_id,
        }
    }
}