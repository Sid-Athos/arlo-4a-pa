use axum::Extension;
use futures_util::TryFutureExt;
use hyper::body::Buf;
use tracing_subscriber::fmt::format;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::game_started_response::GameStartedResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::docker_manager_service::DockerManagerService;
use crate::service::dto::request::init_request::InitRequest;
use crate::service::game_history_service::GameHistoryService;
use crate::service::game_members_history_service::GameMembersHistoryService;
use crate::service::game_move_history_service::GameMoveHistoryService;
use crate::service::lobby_service::LobbyService;


pub struct LaunchGameRequest {}

impl LaunchGameRequest {

    pub async fn compute(pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool.clone());
        let game_history_service = GameHistoryService::new(pool.clone());
        let game_members_history_service = GameMembersHistoryService::new(pool.clone());
        let game_move_history_service = GameMoveHistoryService::new(pool.clone());
        let docker_manager_service = DockerManagerService::new(pool.clone());

        let lobby = lobby_service.get_by_user_id(user.id).await.map_err(status_code_to_string)?;
        let lobby_started = lobby_service.start_game(lobby.id).await.map_err(status_code_to_string)?;
        let lobby_members = lobby_service.get_lobby_member(lobby.id).await.map_err(status_code_to_string)?;
        let game_history = game_history_service.create(lobby.id).await.map_err(status_code_to_string)?;
        lobby_service.set_game_history_id(lobby_started.id, game_history.id).await.map_err(status_code_to_string)?;
        let docker_manager_response;
        if lobby.from_move_history_id.is_some() {
            let last_move = game_move_history_service.get_move_by_id(lobby.from_move_history_id.unwrap()).await.map_err(status_code_to_string)?;
            for i in 1..last_move.action_number {
                let current_move = game_move_history_service.get_move_by_action_number_and_game_history_id(i,last_move.game_history_id).await.map_err(status_code_to_string)?;
                game_move_history_service.create(current_move.player,game_history.id,current_move.game_state,current_move.action).await.map_err(status_code_to_string)?;
            }
            docker_manager_response = docker_manager_service.communicate_docker_manager(user.id, last_move.action).await.map_err(status_code_to_string)?;
        }else {
            let init_request = InitRequest::new(lobby_members.len() as i32);
            docker_manager_response = docker_manager_service.communicate_docker_manager(user.id, serde_json::to_string(&init_request).unwrap()).await.map_err(status_code_to_string)?;

        }
        for lobby_member in &lobby_members {
            let ranking = docker_manager_service.get_ranking(lobby_member.user_id, lobby.game_id).await;
            match ranking {
                Ok(value) => {
                },
                Err(e) => {
                    docker_manager_service.init_rankings(lobby_member.user_id, lobby.game_id).await.map_err(status_code_to_string)?;
                }
            }
        }
        let game_started_response = GameStartedResponse::from_domain(lobby_started, pool.clone()).await?;

        let mut members_user_id = Vec::new();
        for member in &lobby_members {
            game_members_history_service.create(member.user_id, game_history.id, member.player).await.map_err(status_code_to_string)?;
            members_user_id.push(member.user_id);
        }

        connections.send_to_vec_user_id(ResponseEnum::GameStarted(game_started_response), members_user_id).await;

        docker_manager_response.send_to_users(connections, lobby_members).await;

        Ok(())
    }
}
