use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct GameActionsResponse {
    pub player: i32,
    #[serde(rename = "type")]
    pub _type: String,
    pub zones: Vec<HashMap<String, i32>>
}

impl GameActionsResponse {

    pub fn new(player: i32, _type: String, zones: Vec<HashMap<String, i32>>) -> Self {
        GameActionsResponse {
            player,
            _type,
            zones
        }
    }
}