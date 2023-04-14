use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::lobby_service::LobbyService;

#[derive(Deserialize, Debug)]
pub struct JoinLobbyRequest {
    lobby_id: i32
}

impl JoinLobbyRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool);

        let lobby = lobby_service.join_lobby(user.id, self.lobby_id).await?;

        connections.send_to_vec_user_id(ResponseEnum::LobbyJoined, vec![user.id]).await;

        Ok(())
    }
}