use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub min_players: i32,
    pub max_players: i32,
    pub language: String,
    pub code : Option<String>,
    pub user_id : i32,
    pub tag : String
}
