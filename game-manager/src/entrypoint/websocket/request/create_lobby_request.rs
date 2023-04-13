use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateLobbyRequest {
    game_id: i32,
    private: bool
}