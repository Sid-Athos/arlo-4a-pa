use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::ws_session::WsSessionEntity;
use crate::database::mapper::ws_session_entity_mapper::WsSessionEntityMapper;
use crate::domain::model::ws_session::WsSession;

pub struct WsSessionRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl WsSessionRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        WsSessionRepository {
            connection
        }
    }

    pub async fn get_ws_session_with_lobby_id(&self, lobby_id: i32) -> Result<Vec<WsSession>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT * FROM coding_games.ws_session WHERE lobby_id = $1", &[&lobby_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(WsSessionEntityMapper::entity_to_domain(WsSessionEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn get_ws_session_with_user_id(&self, user_id: i32) -> Result<WsSession, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.ws_session WHERE user_id = $1", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = WsSessionEntity::new(row);

        Ok(WsSessionEntityMapper::entity_to_domain(result))
    }

    pub async fn create(&self, lobby_id: i32, user_id: i32) -> Result<WsSession, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.ws_session (lobby_id, user_id) VALUES ($1, $2) RETURNING *", &[&lobby_id, &user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = WsSessionEntity::new(row);

        Ok(WsSessionEntityMapper::entity_to_domain(result))
    }

    pub async fn delete_where_lobby_id(&self, lobby_id: i32) -> Result<Vec<WsSession>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("DELETE FROM coding_games.ws_session WHERE lobby_id = $1 RETURNING *", &[&lobby_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(WsSessionEntityMapper::entity_to_domain(WsSessionEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn delete_where_user_id(&self, user_id: i32) -> Result<WsSession, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("DELETE FROM coding_games.ws_session WHERE user_id = $1 RETURNING *", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = WsSessionEntity::new(row);

        Ok(WsSessionEntityMapper::entity_to_domain(result))
    }
}
