use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::game::Game;
use crate::domain::model::game_move_history::GameMoveHistory;
use crate::service::game_service::GameService;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct GameMoveHistoryResponse {
    pub id: i32,
    pub player : i32,
    pub game_state : String,
    pub action : String,
    pub action_number : i32,
    pub game_history_id : i32
}

impl GameMoveHistoryResponse {

    pub fn from_domain(game_move_history: GameMoveHistory) -> Self {
        GameMoveHistoryResponse {
            id: game_move_history.id,
            player: game_move_history.player,
            game_state: game_move_history.game_state,
            action: game_move_history.action,
            action_number: game_move_history.action_number,
            game_history_id: game_move_history.game_history_id,
        }
    }

    pub fn from_vec_domain(game_moves_history: Vec<GameMoveHistory>) -> Vec<Self> {
        game_moves_history.into_iter().map(|game_move_history| GameMoveHistoryResponse::from_domain(game_move_history)).collect()
    }
}
