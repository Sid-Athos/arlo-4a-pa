use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::games_entity::GamesEntity;

use crate::database::mapper::games_entity_mapper::GamesEntityMapper;

use crate::domain::model::games::Game;



pub struct GamesRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl GamesRepository {
    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        GamesRepository {
            connection
        }
    }


    pub async fn get_all_games(&self) -> Result<Vec<Game>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;
        println!("tamère");
        let rows = conn
            .query("SELECT * FROM coding_games.game", &[])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(GamesEntityMapper::entity_to_domain(GamesEntity::new(row)));
        }

        Ok(result)
    }
}