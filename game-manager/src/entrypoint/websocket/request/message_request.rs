use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::message_response::MessageResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::lobby_service::LobbyService;

#[derive(Deserialize, Debug)]
pub struct MessageRequest {
    pub message: String,
}

impl MessageRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool.clone());
        let lobby = lobby_service.get_by_user_id(user.id).await.map_err(status_code_to_string)?;

        let lobby_response = LobbyResponse::from_domain(lobby, pool).await?;

        let mut members_user_id = Vec::new();
        for member in &lobby_response.members {
            members_user_id.push(member.id);
        }

        let response = MessageResponse {
            from_user: user,
            message: self.message.clone(),
        };

        connections.send_to_vec_user_id(ResponseEnum::Message(response), members_user_id).await;

        Ok(())
    }
}
