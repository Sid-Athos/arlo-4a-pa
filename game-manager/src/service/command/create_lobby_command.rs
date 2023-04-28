
pub struct CreateLobbyCommand {
    pub user_id: i32,
    pub game_id: i32,
    pub private: bool,
}

impl CreateLobbyCommand {

    pub fn new(user_id: i32, game_id: i32, private: bool) -> Self {
        CreateLobbyCommand {
            user_id,
            game_id,
            private
        }
    }
}