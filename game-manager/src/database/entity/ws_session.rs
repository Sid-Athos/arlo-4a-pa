use tokio_postgres::Row;

pub struct WsSessionEntity {
    pub id: i32,
    pub lobby_id: i32,
    pub user_id: i32,
}

impl WsSessionEntity {
    pub fn new(row: Row) -> Self {
        WsSessionEntity {
            id: row.get("id"),
            lobby_id: row.get("lobby_id"),
            user_id: row.get("user_id"),
        }
    }
}
