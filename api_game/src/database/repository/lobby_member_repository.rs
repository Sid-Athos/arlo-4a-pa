use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::lobby_member::LobbyMemberEntity;
use crate::database::mapper::lobby_member_repository::LobbyMemberEntityMapper;
use crate::domain::model::lobby_member::LobbyMember;

pub struct LobbyMemberRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl LobbyMemberRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        LobbyMemberRepository {
            connection
        }
    }

    pub async fn get_by_user_id(&self, user_id: i32) -> Result<LobbyMember, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.lobby_member WHERE user_id = $1", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = LobbyMemberEntity::new(row);

        Ok(LobbyMemberEntityMapper::entity_to_domain(result))
    }

    pub async fn get_by_lobby_id(&self, lobby_id: i32) -> Result<Vec<LobbyMember>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT * FROM coding_games.lobby_member WHERE lobby_id = $1", &[&lobby_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(LobbyMemberEntityMapper::entity_to_domain(LobbyMemberEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn create(&self, user_id: i32, lobby_id: i32, is_host: bool) -> Result<LobbyMember, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.lobby_member (user_id, lobby_id, is_host) VALUES ($1, $2, $3) RETURNING *", &[&user_id, &lobby_id, &is_host])
            .await
            .map_err(database_error_not_found)?;

        let result = LobbyMemberEntity::new(row);

        Ok(LobbyMemberEntityMapper::entity_to_domain(result))
    }

    pub async fn delete(&self, user_id: i32) -> Result<LobbyMember, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("DELETE FROM coding_games.lobby_member WHERE user_id = $1 RETURNING *", &[&user_id])
                            .await
                            .map_err(database_error_not_found)?;

        let result = LobbyMemberEntity::new(row);

        Ok(LobbyMemberEntityMapper::entity_to_domain(result))
    }

    pub async fn update_host(&self, user_id: i32, lobby_id: i32, is_host: bool) -> Result<LobbyMember, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("UPDATE coding_games.lobby_member SET is_host = $1 WHERE user_id = $2 AND lobby_id = $3 RETURNING *", &[&is_host, &user_id, &lobby_id])
            .await
            .map_err(database_error_not_found)?;

        let result = LobbyMemberEntity::new(row);

        Ok(LobbyMemberEntityMapper::entity_to_domain(result))
    }
}