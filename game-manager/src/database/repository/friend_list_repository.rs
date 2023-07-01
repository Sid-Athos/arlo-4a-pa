use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::friend_list_entity::FriendListEntity;
use crate::domain::model::friend_list::FriendList;
use crate::database::mapper::friend_list_entity_mapper::FriendListEntityMapper;

pub struct FriendListRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl FriendListRepository {
    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        FriendListRepository {
            connection
        }
    }

    pub async fn get_friend_list_by_user(&self, user_id: i32) -> Result<Vec<FriendList>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT * FROM coding_games.friend_list WHERE ( recipient_id = $1 OR applicant_id = $1 ) AND accepted = true", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(FriendListEntityMapper::entity_to_domain(FriendListEntity::new(row)));
        }

        Ok(result)
    }





}