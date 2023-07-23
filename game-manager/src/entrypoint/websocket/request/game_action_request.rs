use axum::Extension;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::database::init::ConnectionPool;
use crate::domain::error::{database_error_to_status_code, status_code_to_string};
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::service::docker_manager_service::DockerManagerService;
use crate::service::dto::request::actions_request::{ActionRequest, ActionsRequest};
use crate::service::lobby_service::LobbyService;

#[derive(Deserialize, Serialize, Debug)]
pub struct GameActionRequest {
    x: i32,
    y: i32
}

impl GameActionRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool.clone());
        let docker_manager_service = DockerManagerService::new(pool.clone());

        let lobby = lobby_service.get_by_user_id(user.id).await.map_err(status_code_to_string)?;
        let lobby_members = lobby_service.get_lobby_member(lobby.id).await.map_err(status_code_to_string)?;

        let mut actions_request = ActionsRequest::new(0, 0, 0);

        for lobby_member in &lobby_members {
            if lobby_member.user_id == user.id {
                actions_request = ActionsRequest::new(self.x, self.y, lobby_member.player.clone());
                break;
            }
        }

        if actions_request.actions[0].player == 0 {
            return Err(StatusCode::UNAUTHORIZED.to_string())
        }

        let docker_manager_response = docker_manager_service.communicate_docker_manager(user.id, serde_json::to_string(&actions_request).unwrap()).await.map_err(status_code_to_string)?;
        if docker_manager_response.game_state.game_over {
            for lobby_member in &lobby_members {
                lobby_service.exit_lobby(lobby_member.user_id).await.map_err(status_code_to_string)?;
            }
        }

        docker_manager_response.send_to_users(connections, lobby_members).await;

        return Ok(());
    }
}