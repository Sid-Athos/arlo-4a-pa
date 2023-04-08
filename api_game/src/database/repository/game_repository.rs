use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::game::GameEntity;
use crate::database::mapper::game_entity_mapper::GameEntityMapper;
use crate::domain::model::game::Game;

pub struct GameRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl GameRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        GameRepository {
            connection
        }
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Game, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.game WHERE id = $1", &[&id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }
}