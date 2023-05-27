use tokio_postgres::Row;

pub struct GameEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub min_players: i32,
    pub max_players: i32,
    pub language : String
}

impl GameEntity {
    pub fn new(row: Row) -> Self {
        GameEntity {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            min_players: row.get("min_players"),
            max_players: row.get("max_players"),
            language: row.get("language")
        }
    }
}
