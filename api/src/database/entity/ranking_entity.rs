use tokio_postgres::Row;

pub struct RankingEntity {
    pub id: i32,
    pub user_id: i32,
    pub game_id: i32,
}

impl RankingEntity {
    pub fn new(row: Row) -> Self {
        RankingEntity {
            id: row.get("id"),
            user_id: row.get("user_id"),
            game_id: row.get("game_id"),
        }
    }
}
