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
pub struct AcceptInviteLobbyRequest {
    user_id: i32
}

impl AcceptInviteLobbyRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {

        let invite_service = InviteService::new(pool);

        let invite = invite_service.accept_invite(self.user_id, user.id).await.map_err(status_code_to_string)?;

        let invite_response = InviteResponse::from_domain(invite);

        connections.send_to_vec_user_id(ResponseEnum::InviteLobbyAccepted(invite_response), vec![user.id, self.user_id]).await;

        Ok(())
    }

}