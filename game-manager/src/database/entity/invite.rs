use tokio_postgres::Row;

pub struct InviteEntity {
    pub id: i32,
    pub lobby_id: i32,
    pub from_user_id: i32,
    pub to_user_id: i32,
}

impl InviteEntity {
    pub fn new(row: Row) -> Self {
        InviteEntity {
            id: row.get("id"),
            lobby_id: row.get("lobby_id"),
            from_user_id: row.get("from_user_id"),
            to_user_id: row.get("to_user_id"),
        }
    }
}
