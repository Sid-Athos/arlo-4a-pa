use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommandRequest {
    pub language: String,
    pub commands: Vec<String>,
}

impl CommandRequest {
    pub fn new(language: String, commands: Vec<String>) -> Self {
        CommandRequest {
            language,
            commands
        }
    }
}