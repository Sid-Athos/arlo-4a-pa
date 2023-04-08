use tokio_postgres::Row;

pub struct LobbyMemberEntity {
    pub id: i32,
    pub lobby_id: i32,
    pub user_id: i32,
    pub is_host: bool,
}

impl LobbyMemberEntity {
    pub fn new(row: Row) -> Self {
        LobbyMemberEntity {
            id: row.get("id"),
            lobby_id: row.get("lobby_id"),
            user_id: row.get("user_id"),
            is_host: row.get("is_host"),
        }
    }
}
