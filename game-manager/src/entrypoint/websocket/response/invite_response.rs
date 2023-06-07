use serde::Serialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::invite::Invite;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::user_response::UserResponse;
use crate::service::game_service::GameService;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;

#[derive(Serialize, Debug, Clone)]
pub struct InviteResponse {
    pub from_user: UserResponse,
    pub to_user: UserResponse,
    pub lobby: LobbyResponse,
}

impl InviteResponse {

    pub async fn from_domain(invite: Invite, pool: ConnectionPool) -> Result<Self, String> {

        let user_service = UserService::new(pool.clone());
        let lobby_service = LobbyService::new(pool.clone());

        let from_user = user_service.get_user_by_id(invite.from_user_id).await.map_err(status_code_to_string)?;
        let to_user = user_service.get_user_by_id(invite.to_user_id).await.map_err(status_code_to_string)?;
        let lobby = lobby_service.get_by_id(invite.lobby_id).await.map_err(status_code_to_string)?;

        let from_user_response = UserResponse::from_domain(from_user);
        let to_user_response = UserResponse::from_domain(to_user);
        let lobby_response = LobbyResponse::from_domain(lobby, pool).await?;

        Ok(InviteResponse {
            from_user: from_user_response,
            to_user: to_user_response,
            lobby: lobby_response,
        })
    }
}