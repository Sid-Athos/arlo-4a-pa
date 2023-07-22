use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::game_history::GameHistoryEntity;
use crate::database::mapper::game_history_mapper::GameHistoryEntityMapper;
use crate::domain::model::game::Game;
use crate::domain::model::game_history::GameHistory;

pub struct GameHistoryRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl GameHistoryRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        GameHistoryRepository {
            connection
        }
    }

    pub async fn get_all_by_user_id(&self, user_id : i32) -> Result<Vec<GameHistory>, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT gh.*
            FROM coding_games.game_history gh
            INNER JOIN coding_games.game_members_history gmh ON gh.id = gmh.game_history_id
            WHERE gmh.user_id = $1", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(GameHistoryEntityMapper::entity_to_domain(GameHistoryEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn get_all_by_user_id_and_game_id(&self, user_id: i32, game_id: i32) -> Result<Vec<GameHistory>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT gh.*
            FROM coding_games.game_history gh
            INNER JOIN coding_games.game_members_history gmh ON gh.id = gmh.game_history_id
            WHERE gmh.user_id = $1 AND gh.game_id = $2", &[&user_id, &game_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(GameHistoryEntityMapper::entity_to_domain(GameHistoryEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn create(&self, nb_players : i32, game_id: i32) -> Result<GameHistory, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.game_history \
            (nb_players,game_id) VALUES ($1, $2) RETURNING *", &[&nb_players, &game_id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameHistoryEntity::new(row);

        Ok(GameHistoryEntityMapper::entity_to_domain(result))
    }

    pub async fn get_by_id(&self, id: i32) -> Result<GameHistory, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT * FROM coding_games.game_history WHERE id = $1", &[&id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameHistoryEntity::new(row);

        Ok(GameHistoryEntityMapper::entity_to_domain(result))
    }
}
