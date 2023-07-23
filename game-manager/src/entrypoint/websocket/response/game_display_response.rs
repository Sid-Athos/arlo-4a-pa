use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct GameDisplayResponse {
    pub width: String,
    pub height: String,
    pub content: Vec<HashMap<String, String>>
}

impl GameDisplayResponse {

    pub fn new(width: String, height: String, content: Vec<HashMap<String, String>>) -> Self {
        GameDisplayResponse {
            width,
            height,
            content
        }
    }
}