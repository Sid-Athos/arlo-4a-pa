use tokio_postgres::Row;

pub struct LobbyEntity {
    pub id: i32,
    pub code: String,
    pub game_id: i32,
    pub private: bool,
    pub is_launched: bool,
    pub game_history_id : Option<i32>,
    pub from_move_history_id : Option<i32>
}

impl LobbyEntity {
    pub fn new(row: Row) -> Self {
        LobbyEntity {
            id: row.get("id"),
            code: row.get("code"),
            game_id: row.get("game_id"),
            private: row.get("private"),
            is_launched: row.get("is_launched"),
            game_history_id : row.get("game_history_id"),
            from_move_history_id: row.get("from_move_history_id"),
        }
    }
}
