use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::user::UserEntity;
use crate::database::mapper::user_entity_mapper::UserEntityMapper;
use crate::domain::model::user::User;

pub struct UserRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl UserRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        UserRepository {
            connection
        }
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<User, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.user WHERE id = $1", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::entity_to_domain(result))
    }
}