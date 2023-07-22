use chrono::{DateTime, Utc};
use tokio_postgres::Row;

pub struct GameMoveHistoryEntity {
    pub id: i32,
    pub player : i32,
    pub game_state : String,
    pub action : String,
    pub action_number : i32,
    pub game_history_id : i32
}

impl GameMoveHistoryEntity {
    pub fn new(row: Row) -> Self {
        GameMoveHistoryEntity {
            id: row.get("id"),
            player: row.get("player"),
            game_state: row.get("game_state"),
            action: row.get("action"),
            action_number: row.get("action_number"),
            game_history_id: row.get("game_history_id"),
        }
    }
}
