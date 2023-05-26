use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_duplicate_key, DatabaseError};
use crate::database::entity::game_entity::GameEntity;
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

    pub async fn create_game(&self, name: String, description: Option<String>, min_players : i32, max_players : i32, path : String) -> Result<Game, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.game (name, min_players, max_players, description, path) VALUES ($1, $2, $3, $4, $5) RETURNING *", &[&name, &min_players, &max_players, &description, &path])
            .await
            .map_err(database_error_duplicate_key)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }
}