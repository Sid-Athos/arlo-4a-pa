
pub struct CreateLobbyCommandWithGameHistoryMoveId {
    pub user_id: i32,
    pub game_id: i32,
    pub private: bool,
    pub game_move_history_id : i32
}

impl CreateLobbyCommandWithGameHistoryMoveId {

    pub fn new(user_id: i32, game_id: i32, game_move_history_id : i32) -> Self {
        CreateLobbyCommandWithGameHistoryMoveId {
            user_id,
            game_id,
            private : true,
            game_move_history_id,
        }
    }
}