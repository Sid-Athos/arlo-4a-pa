use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateImageRequest {
    pub language: String,
    pub tag: String,
    pub game_file_name: String
}

impl CreateImageRequest {
    pub fn new(language: String, tag : String, game_file_name : String) -> Self {
        CreateImageRequest {
            language,
            tag,
            game_file_name
        }
    }
}