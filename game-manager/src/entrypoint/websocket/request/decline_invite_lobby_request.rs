use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::invite_response::InviteResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::invite_service::InviteService;

#[derive(Deserialize, Debug)]
pub struct DeclineInviteLobbyRequest {
    user_id: i32
}

impl DeclineInviteLobbyRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {

        let invite_service = InviteService::new(pool.clone());

        let invite = invite_service.cancel_invite(self.user_id, user.id).await.map_err(status_code_to_string)?;

        let invite_response = InviteResponse::from_domain(invite, pool).await?;

        connections.send_to_vec_user_id(ResponseEnum::InviteLobbyDeclined(invite_response), vec![self.user_id, user.id]).await;

        Ok(())
    }
}