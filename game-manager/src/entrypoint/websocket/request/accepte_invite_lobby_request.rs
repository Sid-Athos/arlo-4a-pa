use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::invite_response::InviteResponse;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::entrypoint::websocket::response::user_response::UserResponse;
use crate::service::game_service::GameService;
use crate::service::invite_service::InviteService;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;

#[derive(Deserialize, Debug)]
pub struct AcceptInviteLobbyRequest {
    user_id: i32
}

impl AcceptInviteLobbyRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {

        let invite_service = InviteService::new(pool.clone());

        let invite = invite_service.accept_invite(self.user_id, user.id).await.map_err(status_code_to_string)?;

        let invite_response = InviteResponse::from_domain(invite, pool).await?;

        connections.send_to_vec_user_id(ResponseEnum::InviteLobbyAccepted(invite_response), vec![user.id, self.user_id]).await;
        connections.send_to_vec_user_id(ResponseEnum::LobbyJoined, vec![user.id]).await;

        Ok(())
    }

}