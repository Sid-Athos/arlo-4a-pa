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
pub struct GiveHostRequest {
    user_id: i32
}

impl GiveHostRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {

        let lobby_service = LobbyService::new(pool.clone());

        let user_service = UserService::new(pool);

        let lobby_member = lobby_service.get_lobby_member_by_user_id(user.id).await.map_err(status_code_to_string)?;

        if !lobby_member.is_host {
            return Err("You are not the host of this lobby".to_string());
        }

        user_service.get_user_by_id(self.user_id).await.map_err(status_code_to_string)?;
        lobby_service.give_host(user.id, self.user_id, lobby_member.lobby_id).await.map_err(status_code_to_string)?;

        connections.send_to_vec_user_id(ResponseEnum::LobbyHostGiven, vec![user.id]).await;
        connections.send_to_vec_user_id(ResponseEnum::LobbyHostTaken, vec![self.user_id]).await;

        Ok(())
    }
}