use crate::database::entity::user_entity::UserEntity;
use crate::domain::model::user::User;

pub struct UserEntityMapper;

impl UserEntityMapper {

    pub fn to_user_entity(user: User) -> UserEntity {
        UserEntity {
            id: user.id,
            pseudo: user.pseudo,
            email: user.email,
            password: user.password
        }
    }

    pub fn to_user(user_entity: UserEntity) -> User {
        User {
            id: user_entity.id,
            pseudo: user_entity.pseudo,
            email: user_entity.email,
            password: user_entity.password
        }
    }
}