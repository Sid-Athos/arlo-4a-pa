use chrono::{DateTime, Utc};
use tokio_postgres::Row;

pub struct GameMembersHistoryEntity {
    pub id: i32,
    pub user_id : i32,
    pub game_history_id : i32,
    pub player : i32
}

impl GameMembersHistoryEntity {
    pub fn new(row: Row) -> Self {
        GameMembersHistoryEntity {
            id: row.get("id"),
            user_id: row.get("user_id"),
            game_history_id: row.get("game_history_id"),
            player: row.get("player"),
        }
    }
}
