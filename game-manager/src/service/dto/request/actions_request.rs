use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ActionsRequest {
    pub actions: Vec<ActionRequest>
}

#[derive(Serialize, Deserialize)]
pub struct ActionRequest {
    pub x: i32,
    pub y: i32,
    pub player: i32
}

impl ActionsRequest {

    pub fn new(x: i32, y: i32, player: i32) -> Self {
        ActionsRequest {
            actions: vec![
                ActionRequest {
                    x,
                    y,
                    player
                }
            ]
        }
    }
}