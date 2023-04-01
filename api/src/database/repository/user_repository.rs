use axum::http::StatusCode;
use bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{NoTls, Row};
use crate::database::entity::user_entity::UserEntity;
use crate::database::mapper::user_entity_mapper::UserEntityMapper;
use crate::domain::error::internal_error;
use crate::domain::model::user::User;

pub struct UserRepository {
    pub connection: PooledConnection<'static, PostgresConnectionManager<NoTls>>,
}

impl UserRepository {

    pub fn new(connection: PooledConnection<'static, PostgresConnectionManager<NoTls>>) -> Self {
        UserRepository {
            connection
        }
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<User, (StatusCode, String)> {

        let row: Row = self.connection
            .query_one("SELECT * FROM coding_games.user WHERE id = $1", &[&user_id])
            .await
            .map_err(internal_error)?;

        let result = UserEntity::new(row);

        Ok(UserEntityMapper::to_user(result))
    }
}
