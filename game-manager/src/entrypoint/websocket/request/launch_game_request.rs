use axum::Extension;
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

        let game_started_response = GameStartedResponse::from_domain(lobby.clone(), pool.clone()).await?;

        let lobby_response = LobbyResponse::from_domain(lobby, pool).await?;

        let mut members_user_id = Vec::new();
        for member in &lobby_response.members {
            members_user_id.push(member.id);
        }

        if (lobby_response.members.len() as i32) >= lobby_response.game.min_players &&
            (lobby_response.members.len() as i32) <= lobby_response.game.max_players {
            connections.send_to_vec_user_id(ResponseEnum::GameStarted(game_started_response), members_user_id).await;
        } else {
            connections.send_to_vec_user_id(ResponseEnum::CannotStartGame, vec![user.id]).await;
        }

        Ok(())
    }
}
