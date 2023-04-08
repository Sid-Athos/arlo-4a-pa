use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::session::SessionEntity;
use crate::database::mapper::session_entity_mapper::SessionEntityMapper;
use crate::domain::model::session::Session;

pub struct SessionRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl SessionRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        SessionRepository {
            connection
        }
    }

    pub async fn get_by_token(&self, token: String) -> Result<Session, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.session WHERE token = $1", &[&token])
            .await
            .map_err(database_error_not_found)?;

        let result = SessionEntity::new(row);

        Ok(SessionEntityMapper::entity_to_domain(result))
    }
}