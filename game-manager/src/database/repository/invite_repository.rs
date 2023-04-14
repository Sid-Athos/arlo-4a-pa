use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::invite::InviteEntity;
use crate::database::mapper::invite_entity_mapper::InviteEntityMapper;
use crate::domain::model::invite::Invite;

pub struct InviteRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl InviteRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        InviteRepository {
            connection
        }
    }

    pub async fn create(&self, lobby_id: i32, from_user_id: i32, to_user_id: i32) -> Result<Invite, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.invite (lobby_id, from_user_id, to_user_id) VALUES ($1, $2, $3) RETURNING *", &[&lobby_id, &from_user_id, &to_user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = InviteEntity::new(row);

        Ok(InviteEntityMapper::entity_to_domain(result))
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Invite, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.invite WHERE id = $1", &[&id])
            .await
            .map_err(database_error_not_found)?;

        let result = InviteEntity::new(row);

        Ok(InviteEntityMapper::entity_to_domain(result))
    }

    pub async fn get_by_users_id_and_lobby_id(&self, from_user_id: i32, to_user_id: i32, lobby_id: i32) -> Result<Invite, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.invite WHERE to_user_id = $1 AND from_user_id = $2 AND lobby_id = $3", &[&to_user_id, &from_user_id, &lobby_id])
            .await
            .map_err(database_error_not_found)?;

        let result = InviteEntity::new(row);

        Ok(InviteEntityMapper::entity_to_domain(result))
    }

    pub async fn delete_by_id(&self, id: i32) -> Result<Invite, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("DELETE FROM coding_games.invite WHERE id = $1 RETURNING *", &[&id])
            .await
            .map_err(database_error_not_found)?;

        let result = InviteEntity::new(row);

        Ok(InviteEntityMapper::entity_to_domain(result))
    }

    pub async fn delete_all_for_lobby_id(&self, lobby_id: i32) -> Result<Vec<Invite>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("DELETE FROM coding_games.invite WHERE lobby_id = $1 RETURNING *", &[&lobby_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(InviteEntityMapper::entity_to_domain(InviteEntity::new(row)));
        }

        Ok(result)
    }
}