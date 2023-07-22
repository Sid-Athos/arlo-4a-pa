use tokio_postgres::Row;

pub struct GameHistoryEntity {
    pub id: i32,
    pub date_time : String,
    pub nb_players : i32,
    pub game_id : i32
}

impl GameHistoryEntity {
    pub fn new(row: Row) -> Self {
        GameHistoryEntity {
            id: row.get("id"),
            date_time: row.get("date_time"),
            nb_players: row.get("nb_players"),
            game_id: row.get("game_id"),
        }
    }
}
