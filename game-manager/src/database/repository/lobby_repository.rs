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

    pub async fn get_by_id(&self, lobby_id: i32) -> Result<Lobby, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.lobby WHERE id = $1", &[&lobby_id])
            .await
            .map_err(database_error_not_found)?;

        let result = LobbyEntity::new(row);

        Ok(LobbyEntityMapper::entity_to_domain(result))
    }

    pub async fn get_public(&self) -> Result<Vec<Lobby>, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT * FROM coding_games.lobby WHERE private = false AND is_launched = false", &[])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(LobbyEntityMapper::entity_to_domain(LobbyEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn get_public_by_game_id(&self, game_id: i32) -> Result<Vec<Lobby>, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT * FROM coding_games.lobby WHERE private = false AND is_launched = false AND game_id = $1", &[&game_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(LobbyEntityMapper::entity_to_domain(LobbyEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn create_lobby(&self, code: String, game_id: i32, private: bool) -> Result<Lobby, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.lobby (code, game_id, private) VALUES ($1, $2, $3) RETURNING *", &[&code, &game_id, &private])
            .await
            .map_err(database_error_not_found)?;

        let result = LobbyEntity::new(row);

        Ok(LobbyEntityMapper::entity_to_domain(result))
    }

    pub async fn delete_lobby(&self, lobby_id: i32) -> Result<Lobby, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("DELETE FROM coding_games.lobby WHERE id = $1 RETURNING *", &[&lobby_id])
            .await
            .map_err(database_error_not_found)?;

        let result = LobbyEntity::new(row);

        Ok(LobbyEntityMapper::entity_to_domain(result))
    }

    pub async fn get_by_code(&self, code: String) -> Result<Lobby, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.lobby WHERE code = $1", &[&code])
            .await
            .map_err(database_error_not_found)?;

        let result = LobbyEntity::new(row);

        Ok(LobbyEntityMapper::entity_to_domain(result))
    }

    pub async fn set_launch_for_lobby_id(&self, lobby_id: i32) -> Result<Lobby, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("UPDATE coding_games.lobby SET is_launched = true WHERE lobby_id = $1 RETURNING *", &[&lobby_id])
            .await
            .map_err(database_error_not_found)?;

        let result = LobbyEntity::new(row);

        Ok(LobbyEntityMapper::entity_to_domain(result))
    }
}