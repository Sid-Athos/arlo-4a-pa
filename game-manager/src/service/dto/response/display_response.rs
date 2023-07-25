use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DisplayResponse {
    pub width: String,
    pub height: String,
    pub player: i32,
    pub content: Vec<HashMap<String, String>>
}