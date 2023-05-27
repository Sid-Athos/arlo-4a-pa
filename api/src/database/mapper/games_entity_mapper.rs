use crate::database::entity::games_entity::GamesEntity;
use crate::database::entity::user_entity::UserEntity;
use crate::domain::model::games::Game;
use crate::domain::model::user::User;

pub struct GamesEntityMapper;

impl GamesEntityMapper {

    pub fn entity_to_domain(games_entity: GamesEntity) -> Game {
        Game {
            id: games_entity.id,
            min_players: games_entity.min_players,
            max_players: games_entity.max_players,
            description: games_entity.description,
        }
    }
}