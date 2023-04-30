use crate::database::entity::game::GameEntity;
use crate::domain::model::game::Game;

pub struct GameEntityMapper;

impl GameEntityMapper {

    pub fn entity_to_domain(lobby_entity: GameEntity) -> Game {
        Game {
            id: lobby_entity.id,
            name: lobby_entity.name,
            nb_player: lobby_entity.nb_player,
        }
    }
}