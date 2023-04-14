use axum::Extension;
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::lobby_service::LobbyService;

pub struct ExitLobbyRequest {}

impl ExitLobbyRequest {
    pub async fn compute(pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool);

        let lobby = lobby_service.exit_lobby(user.id).await?;

        connections.send_to_vec_user_id(ResponseEnum::LobbyExited, vec![user.id]).await;

        Ok(())
    }
}