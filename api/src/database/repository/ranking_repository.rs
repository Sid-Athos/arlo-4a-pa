use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use crate::database::database_error::{database_error_cannot_get_connection_to_database, database_error_duplicate_key, database_error_not_found, DatabaseError};
use crate::database::entity::ranking_entity::RankingEntity;
use crate::database::mapper::ranking_entity_mapper::RankingEntityMapper;
use crate::domain::model::ranking::Ranking;

pub struct RankingRepository {
    pub connection: Pool<PostgresConnectionManager<NoTls>>,
}

impl RankingRepository {
    pub fn new(connection: Pool<PostgresConnectionManager<NoTls>>) -> Self {
        RankingRepository {
            connection
        }
    }

    pub async fn create_ranking(&self, game_id: i32, user_id: i32) -> Result<Ranking, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("INSERT INTO coding_games.ranking (game_id, user_id) VALUES ($1, $2) RETURNING *", &[&game_id, &user_id])
            .await
            .map_err(database_error_duplicate_key)?;

        let result = RankingEntity::new(row);

        Ok(RankingEntityMapper::entity_to_domain(result))
    }

    pub async fn get_one_ranking(&self, game_id: i32, user_id: i32) -> Result<Ranking, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("SELECT r.* FROM coding_games.ranking r WHERE game_id = $1 AND user_id = $2", &[&game_id, &user_id])
            .await
            .map_err(database_error_duplicate_key)?;

        let result = RankingEntity::new(row);

        Ok(RankingEntityMapper::entity_to_domain(result))
    }

    pub async fn update_ranking(&self, game_id: i32, user_id: i32, new_ranking : i32) -> Result<Ranking, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let row = conn
            .query_one("UPDATE coding_games.ranking SET rank = $1,nb_games = nb_games + 1 WHERE game_id = $2 AND user_id = $3 RETURNING *", &[&new_ranking,&game_id, &user_id])
            .await
            .map_err(database_error_not_found)?;

        let result = RankingEntity::new(row);

        Ok(RankingEntityMapper::entity_to_domain(result))
    }

    pub async fn delete_rankings_by_user_id(&self, user_id: i32) -> Result<Vec<Ranking>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("DELETE FROM coding_games.ranking WHERE user_id = $1 RETURNING *", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingEntityMapper::entity_to_domain(RankingEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn delete_rankings_by_game_id(&self,  game_id: i32) -> Result<Vec<Ranking>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("DELETE FROM coding_games.ranking WHERE game_id = $1 RETURNING *", &[&game_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingEntityMapper::entity_to_domain(RankingEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn get_ranking_by_friends(&self, user_id: i32, game_id : i32) -> Result<Vec<Ranking>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT r.*
                        FROM coding_games.ranking r
                 JOIN coding_games.user u ON u.id = r.user_id
                 JOIN coding_games.game g ON g.id = r.game_id
                    WHERE u.id IN (
                        SELECT DISTINCT CASE
                                            WHEN fl.applicant_id = $1 THEN fl.recipient_id
                                            WHEN fl.recipient_id = $1 THEN fl.applicant_id
                                            END AS friend_id
                        FROM coding_games.friend_list fl
                        WHERE fl.applicant_id = $1 OR fl.recipient_id = $1 AND fl.accepted = true
                        )
                      AND g.id = $2", &[&user_id,&game_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingEntityMapper::entity_to_domain(RankingEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn get_ranking_by_game(&self, game_id : i32) -> Result<Vec<Ranking>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT r.* FROM coding_games.ranking r WHERE game_id = $1", &[&game_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingEntityMapper::entity_to_domain(RankingEntity::new(row)));
        }

        Ok(result)
    }

    pub async fn get_ranking_by_user(&self, user_id : i32) -> Result<Vec<Ranking>, DatabaseError> {

        let conn = self.connection.get().await.map_err(database_error_cannot_get_connection_to_database)?;

        let rows = conn
            .query("SELECT r.* FROM coding_games.ranking r WHERE user_id = $1", &[&user_id])
            .await
            .map_err(database_error_not_found)?;

        let mut result = Vec::new();

        for row in rows {
            result.push(RankingEntityMapper::entity_to_domain(RankingEntity::new(row)));
        }

        Ok(result)
    }



}