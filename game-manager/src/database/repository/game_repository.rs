use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use uuid::{uuid, Uuid};
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
            .query_one("SELECT id,name,min_players,max_players,description,language,user_id,tag FROM coding_games.game WHERE id = $1", &[&id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new_without_code(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }

    pub async fn get_my_games(&self, id: i32) -> Result<Vec<Game>, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;
        tracing::info!("Init db get all");
        let rows = conn
            .query("SELECT id,name,min_players,max_players,description,language,code,user_id, tag FROM coding_games.game WHERE user_id = $1", &[&id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(GameEntityMapper::entity_to_domain(GameEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn get_all(&self) -> Result<Vec<Game>, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;
        tracing::info!("Init db get all");
        let rows = conn
            .query("SELECT * FROM coding_games.game", &[])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(GameEntityMapper::entity_to_domain(GameEntity::new_without_code(row)));
        }

        Ok(result)
    }

    pub async fn get_game_and_code_by_id(&self, id: i32) -> Result<Game, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.game WHERE id = $1", &[&id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }

    pub async fn create(&self, name : String, min_players: i32, max_players : i32, description : Option<String> , language : String, user_id : i32, code : String, tag : String) -> Result<Game, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;


        let row = conn
            .query_one("INSERT INTO coding_games.game (name, min_players, max_players, description, language, user_id, code, tag) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *", &[&name, &min_players, &max_players, &description, &language, &user_id, &code, &tag])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }

    pub async fn update_code(&self, code : String, id : i32) -> Result<Game, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("UPDATE coding_games.game SET code = $1 WHERE id = $2 RETURNING *", &[&code, &id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }

    pub async fn update_name(&self, name : String, id : i32) -> Result<Game, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("UPDATE coding_games.game SET name = $1 WHERE id = $2 RETURNING *", &[&name, &id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }

    pub async fn update_min_players(&self, min_players : i32, id : i32) -> Result<Game, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("UPDATE coding_games.game SET min_players = $1 WHERE id = $2 RETURNING *", &[&min_players, &id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }

    pub async fn update_max_players(&self, max_players : i32, id : i32) -> Result<Game, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("UPDATE coding_games.game SET max_players = $1 WHERE id = $2 RETURNING *", &[&max_players, &id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }

    pub async fn update_description(&self, description : String, id : i32) -> Result<Game, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("UPDATE coding_games.game SET description = $1 WHERE id = $2 RETURNING *", &[&description, &id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }

    pub async fn update_language(&self, language : String, id : i32) -> Result<Game, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("UPDATE coding_games.game SET language = $1 WHERE id = $2 RETURNING *", &[&language, &id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }

    pub async fn delete(&self, id : i32) -> Result<Game, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn.query_one("DELETE FROM coding_games.game WHERE id = $1 RETURNING *", &[&id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameEntity::new(row);

        Ok(GameEntityMapper::entity_to_domain(result))
    }
}
