use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::game_members_history::GameMembersHistoryEntity;
use crate::database::entity::game_move_history::GameMoveHistoryEntity;
use crate::database::mapper::game_members_history_mapper::GameMembersHistoryEntityMapper;
use crate::database::mapper::game_move_history_mapper::GameMoveHistoryEntityMapper;
use crate::domain::model::game::Game;
use crate::domain::model::game_members_history::GameMembersHistory;
use crate::domain::model::game_move_history::GameMoveHistory;

pub struct GameMoveHistoryRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl GameMoveHistoryRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        GameMoveHistoryRepository {
            connection
        }
    }

    pub async fn get_all_by_game_history_id(&self, game_history_id: i32) -> Result<Vec<GameMoveHistory>, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;
        tracing::info!("Init db get all");
        let rows = conn
            .query("SELECT *
            FROM coding_games.game_move_history
            WHERE game_history_id = $1 ORDER BY action_number", &[&game_history_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(GameMoveHistoryEntityMapper::entity_to_domain(GameMoveHistoryEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn create(&self, player : i32, game_state: String, action : String, game_history_id : i32) -> Result<GameMoveHistory, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.game_move_history \
            (player,game_state,action,action_number,game_history_id) \
            VALUES \
            ($1,$2,$3,(COALESCE( (SELECT MAX(action_number) \
            FROM coding_games.game_move_history WHERE game_history_id = $4), 0))+1, $4) RETURNING *", &[&player, &game_state, &action, &game_history_id])
            .await
            .map_err(database_error_not_found)?;

        let result = GameMoveHistoryEntity::new(row);

        Ok(GameMoveHistoryEntityMapper::entity_to_domain(result))
    }
}