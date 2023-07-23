use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ActionsResponse {
    pub player: i32,
    #[serde(rename = "type")]
    pub _type: String,
    pub zones: Vec<HashMap<String, i32>>
}