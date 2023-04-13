use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::model::lobby::Lobby;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::command::create_lobby_command::CreateLobbyCommand;
use crate::service::lobby_service::LobbyService;

#[derive(Deserialize, Debug)]
pub struct CreateLobbyRequest {
    game_id: i32,
    private: bool
}

impl CreateLobbyRequest {
    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) {

        let lobby_service = LobbyService::new(
            GameRepository::new(pool.clone()),
            LobbyRepository::new(pool.clone()),
            LobbyMemberRepository::new(pool.clone()),
        );

        let command = CreateLobbyCommand::new(user.id, self.game_id, self.private);

        let lobby = lobby_service.create(command).await;

        if lobby.is_err() {
            connections.send_to_vec_user_id(ResponseEnum::Error(lobby.err().unwrap().to_string()), vec![user.id]).await;;
        } else {
            connections.send_to_vec_user_id(ResponseEnum::LobbyCreated, vec![user.id]).await;
        }
    }
}