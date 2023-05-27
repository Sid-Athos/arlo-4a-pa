use crate::database::entity::game::GameEntity;
use crate::domain::model::game::Game;

pub struct GameEntityMapper;

impl GameEntityMapper {

    pub fn entity_to_domain(game_entity: GameEntity) -> Game {
        Game {
            id: game_entity.id,
            name: game_entity.name,
            description: game_entity.description,
            min_players: game_entity.min_players,
            max_players: game_entity.max_players,
            language : game_entity.language
        }
    }
}