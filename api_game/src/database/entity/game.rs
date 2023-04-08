use tokio_postgres::Row;

pub struct GameEntity {
    pub id: i32,
    pub name: String,
    pub nb_player: i32,
}

impl GameEntity {
    pub fn new(row: Row) -> Self {
        GameEntity {
            id: row.get("id"),
            name: row.get("name"),
            nb_player: row.get("nb_player"),
        }
    }
}
