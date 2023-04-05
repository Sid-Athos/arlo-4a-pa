use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::lobby::LobbyEntity;
use crate::database::mapper::lobby_entity_mapper::LobbyEntityMapper;
use crate::domain::model::lobby::Lobby;

pub struct LobbyRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl LobbyRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        LobbyRepository {
            connection
        }
    }

    pub async fn get_public(&self) -> Result<Vec<Lobby>, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT * FROM coding_games.lobby WHERE private = false", &[])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(LobbyEntityMapper::entity_to_domain(LobbyEntity::new(row)));
        }

        Ok(result)
    }
}