pub struct UpdateGameCommand {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub min_players : Option<i32>,
    pub max_players : Option<i32>,
    pub code : Option<String>
}

impl UpdateGameCommand {

    pub fn new(id: i32, name: Option<String>, description: Option<String>, language: Option<String>, min_players : Option<i32>, max_players : Option<i32>, code : Option<String>) -> Self {
        UpdateGameCommand {
            id,
            name,
            description,
            language,
            min_players,
            max_players,
            code
        }
    }
}