use tokio_postgres::Row;

pub struct GamesEntity {
    pub id: i32,
    pub min_players: i32,
    pub max_players: i32,
    pub description : String,
    pub path : String
}

impl GamesEntity {
    pub fn new(row: Row) -> Self {
        GamesEntity {
            id: row.get("id"),
            min_players: row.get("min_players"),
            max_players: row.get("max_players"),
            description: row.get("description"),
            path: row.get("path")
        }
    }
}