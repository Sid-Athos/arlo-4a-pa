use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;

#[derive(Deserialize, Debug)]
pub struct KickUserRequest {
    user_id: i32
}

impl KickUserRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool.clone());

        let user_service = UserService::new(pool);

        let user_to_kick = user_service.get_user_by_id(self.user_id).await.map_err(status_code_to_string)?;

        if user_to_kick.id == user.id {
            return Err("You can't kick yourself".to_string());
        }

        let lobby_member = lobby_service.get_lobby_member_by_user_id(user.id).await.map_err(status_code_to_string)?;

        lobby_service.kick( lobby_member.lobby_id, self.user_id).await.map_err(status_code_to_string)?;

        connections.send_to_vec_user_id(ResponseEnum::UserKicked, vec![user.id]).await;
        connections.send_to_vec_user_id(ResponseEnum::Kicked, vec![self.user_id]).await;

        Ok(())
    }
}