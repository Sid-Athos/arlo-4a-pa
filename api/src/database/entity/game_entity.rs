use tokio_postgres::Row;

pub struct GameEntity {
    pub id: i32,
    pub name: String,
    pub max_players: i32,
    pub min_players: i32,
    pub description: String,
    pub path : String
}

impl GameEntity {
    pub fn new(row: Row) -> Self {
        GameEntity {
            id: row.get("id"),
            name: row.get("name"),
            max_players: row.get("max_players"),
            min_players: row.get("min_players"),
            description: row.get("description"),
            path : row.get("path")
        }
    }
}
