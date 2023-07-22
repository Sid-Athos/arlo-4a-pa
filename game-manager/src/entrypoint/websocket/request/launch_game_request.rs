use axum::Extension;
use futures_util::TryFutureExt;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::game_started_response::GameStartedResponse;
use crate::entrypoint::websocket::response::lobby_response::LobbyResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::lobby_service::LobbyService;


pub struct LaunchGameRequest {}

impl LaunchGameRequest {

    pub async fn compute(pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool.clone());
        let lobby = lobby_service.get_by_user_id(user.id).await.map_err(status_code_to_string)?;
        let lobby_started = lobby_service.start_game(lobby.id).await.map_err(status_code_to_string)?;
        let lobby_members = lobby_service.get_lobby_member(lobby.id).await.map_err(status_code_to_string)?;

        let game_started_response = GameStartedResponse::from_domain(lobby_started, pool.clone()).await?;

        let mut members_user_id = Vec::new();
        for member in &lobby_members {
            members_user_id.push(member.user_id);
        }

        connections.send_to_vec_user_id(ResponseEnum::GameStarted(game_started_response), members_user_id).await;

        Ok(())
    }
}
