use axum::Extension;
use serde::Deserialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::emote_response::EmoteResponse;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::lobby_service::LobbyService;

#[derive(Deserialize, Debug)]
pub struct EmoteRequest {
    pub emote: String,
}

impl EmoteRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool.clone());
        let lobby = lobby_service.get_by_user_id(user.id).await.map_err(status_code_to_string)?;

        let lobby_response = LobbyResponse::from_domain(lobby, pool).await?;

        let mut members_user_id = Vec::new();
        for member in &lobby_response.members {
            members_user_id.push(member.id);
        }

        let response = EmoteResponse {
            from_user: user,
            emote: self.emote.clone(),
        };

        connections.send_to_vec_user_id(ResponseEnum::Emote(response), members_user_id).await;

        Ok(())
    }
}
