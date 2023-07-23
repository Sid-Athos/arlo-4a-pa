use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::database::init::ConnectionPool;
use crate::domain::model::game_history::GameHistory;
use crate::entrypoint::history::route::response::game_response::GameResponse;
use crate::service::game_service::GameService;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct GameHistoryResponse {
    pub id: i32,
    pub date_time: DateTime<Utc>,
    pub nb_players: i32,
    pub game: GameResponse,
}

impl GameHistoryResponse {

    pub async fn from_domain(game_history: GameHistory, pool: ConnectionPool) -> Result<Self, StatusCode> {
        let game_service = GameService::new(pool);
        Ok(
            GameHistoryResponse {
                id: game_history.id,
                date_time: game_history.date_time.into(),
                nb_players: game_history.nb_players,
                game: GameResponse::from_domain(game_service.get_by_id(game_history.game_id).await?),
            }
        )
    }
}
