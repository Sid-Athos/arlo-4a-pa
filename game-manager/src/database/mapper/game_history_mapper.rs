use crate::database::entity::game::GameEntity;
use crate::database::entity::game_history::GameHistoryEntity;
use crate::domain::model::game::Game;
use crate::domain::model::game_history::GameHistory;

pub struct GameHistoryEntityMapper;

impl GameHistoryEntityMapper {

    pub fn entity_to_domain(game_history_entity: GameHistoryEntity) -> GameHistory {
        GameHistory {
            id: game_history_entity.id,
            date_time: game_history_entity.date_time,
            nb_players: game_history_entity.nb_players,
            game_id: game_history_entity.game_id,
        }
    }
}