use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::command::create_lobby_command::CreateLobbyCommand;
use crate::service::command::create_lobby_command_with_game_history_move_id::CreateLobbyCommandWithGameHistoryMoveId;
use crate::service::game_move_history_service::GameMoveHistoryService;
use crate::service::lobby_service::LobbyService;

#[derive(Deserialize, Debug)]
pub struct CreateLobbyByGameMoveHistoryIdRequest {
    game_move_id: i32,
}

impl CreateLobbyByGameMoveHistoryIdRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {

        let lobby_service = LobbyService::new(pool.clone());
        let game_move_history_service = GameMoveHistoryService::new(pool.clone());

        let game_id = game_move_history_service.get_game_id(self.game_move_id).await.map_err(status_code_to_string)?;
        let command = CreateLobbyCommandWithGameHistoryMoveId::new(user.id,game_id , self.game_move_id);

        lobby_service.create_from_move_history_id(command).await.map_err(status_code_to_string)?;

        connections.send_to_vec_user_id(ResponseEnum::LobbyCreated, vec![user.id]).await;

        Ok(())
    }
}