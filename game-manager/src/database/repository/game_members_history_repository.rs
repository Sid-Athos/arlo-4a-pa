use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_not_found, DatabaseError};
use crate::database::entity::game::GameMembersHistoryEntity;
use crate::database::entity::game_history::GameMembersHistoryEntity;
use crate::database::entity::game_members_history::GameMembersHistoryEntity;
use crate::database::mapper::game_entity_mapper::GameMembersHistoryEntityMapper;
use crate::database::mapper::game_history_mapper::GameMembersHistoryEntityMapper;
use crate::database::mapper::game_members_history_mapper::GameMembersHistoryEntityMapper;
use crate::domain::model::game::Game;
use crate::domain::model::game_history::GameMembersHistory;
use crate::domain::model::game_members_history::GameMembersHistory;

pub struct GameMembersHistoryRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl GameMembersHistoryRepository {

    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        GameMembersHistoryRepository {
            connection
        }
    }

    pub async fn get_all_by_game_history_id(&self, game_history_id: i32) -> Result<Vec<GameMembersHistory>, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;
        tracing::info!("Init db get all");
        let rows = conn
            .query("SELECT *
            FROM coding_games.game_members_history
            WHERE game_history_id = $1", &[&game_history_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(GamMemberseHistoryEntityMapper::entity_to_domain(GameMembersHistoryEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn create(&self, user_id : i32, game_history_id: i32, player : i32) -> Result<GameMembersHistory, DatabaseError> {
        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.game_members_history \
            (user_id,game_history_id,player) VALUES ($1, $2, $3) RETURNING *", &[&user_id, &game_history_id, &player])
            .await
            .map_err(database_error_not_found)?;

        let result = GameMembersHistoryEntity::new(row);

        Ok(GameMembersHistoryEntityMapper::entity_to_domain(result))
    }
}
