use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GameDto {
    pub language: String,
    pub commands: Vec<String>,
}

impl GameDto {
    pub fn new(language: String, commands: Vec<String>) -> Self {
        GameDto {
            language,
            commands
        }
    }
}