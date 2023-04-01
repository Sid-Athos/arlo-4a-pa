use axum::http::StatusCode;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::entity::session_entity::SessionEntity;
use crate::database::mapper::session_entity_mapper::SessionEntityMapper;
use crate::domain::error::internal_error;
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

    pub async fn get_by_token(&self, token: String) -> Result<Session, (StatusCode, String)> {

        let conn = self.connection.get().await.map_err(internal_error)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.session WHERE token = $1", &[&token])
            .await
            .map_err(internal_error)?;

        let result = SessionEntity::new(row);

        Ok(SessionEntityMapper::entity_to_domain(result))
    }

    pub async fn create_session(&self, user_id: i32, token: String) -> Result<Session, (StatusCode, String)> {

        let conn = self.connection.get().await.map_err(internal_error)?;

        let row = conn
            .query_one("INSERT INTO coding_games.session (user_id, token) VALUES ($1, $2) RETURNING *", &[&user_id, &token])
            .await
            .map_err(internal_error)?;

        let result = SessionEntity::new(row);

        Ok(SessionEntityMapper::entity_to_domain(result))
    }

    pub async fn delete_token(&self, token: String) -> Result<Session, (StatusCode, String)> {

        let conn = self.connection.get().await.map_err(internal_error)?;

        let row = conn.query_one("DELETE FROM coding_games.session WHERE token = $1 RETURNING *", &[&token])
            .await
            .map_err(internal_error)?;

        let result = SessionEntity::new(row);

        Ok(SessionEntityMapper::entity_to_domain(result))
    }
}
