use tokio_postgres::Row;

pub struct LobbyEntity {
    pub id: i32,
    pub code: String,
    pub game_id: i32,
    pub private: bool,
}

impl LobbyEntity {
    pub fn new(row: Row) -> Self {
        LobbyEntity {
            id: row.get("id"),
            code: row.get("code"),
            game_id: row.get("game_id"),
            private: row.get("private"),
        }
    }
}
