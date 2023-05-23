use axum::Extension;
use crate::database::init::ConnectionPool;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::invite_response::InviteResponse;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::invite_service::InviteService;
use crate::service::lobby_service::LobbyService;

pub struct ExitLobbyRequest {}

impl ExitLobbyRequest {

    pub async fn compute(pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool.clone());
        let invite_service = InviteService::new(pool.clone());

        let lobby = lobby_service.exit_lobby(user.id).await?;
        let invites = invite_service.cancel_all_from_user_id(user.id).await?;

        for invite in invites {
            let invite_response = InviteResponse::from_domain(invite.clone());
            connections.send_to_vec_user_id(ResponseEnum::InviteLobbyCancelled(invite_response), vec![invite.to_user_id]).await;
        }

        connections.send_to_vec_user_id(ResponseEnum::LobbyExited, vec![user.id]).await;
        LobbyResponse::send_lobby_to_members_from_lobby_id(pool, connections, lobby.id).await?;

        Ok(())
    }
}