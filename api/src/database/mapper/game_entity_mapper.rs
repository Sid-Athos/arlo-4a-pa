use crate::database::entity::game_entity::GameEntity;
use crate::domain::model::game::Game;

pub struct GameEntityMapper;

impl GameEntityMapper {

    pub fn entity_to_domain(game_entity: GameEntity) -> Game {
        Game {
            id: game_entity.id,
            name : game_entity.name,
            max_players: game_entity.max_players,
            min_players: game_entity.min_players,
            description: game_entity.description,
            path : game_entity.path
        }
    }
}