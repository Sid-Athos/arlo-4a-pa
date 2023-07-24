use axum::Extension;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::database::init::ConnectionPool;
use crate::domain::error::{database_error_to_status_code, status_code_to_string};
use crate::domain::model::lobby::Lobby;
use crate::domain::model::lobby_member::LobbyMember;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::service::docker_manager_service::DockerManagerService;
use crate::service::dto::request::actions_request::{ActionRequest, ActionsRequest};
use crate::service::dto::response::docker_manager_response::DockerManagerResponse;
use crate::service::game_move_history_service::GameMoveHistoryService;
use crate::service::lobby_service::LobbyService;

#[derive(Deserialize, Serialize, Debug)]
pub struct GameActionRequest {
    x: i32,
    y: i32
}

impl GameActionRequest {

    pub async fn compute(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool.clone());
        let game_move_history_service = GameMoveHistoryService::new(pool.clone());

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

        let last_move = game_move_history_service.get_last_by_game_history_id(lobby.game_history_id.unwrap()).await.map_err(status_code_to_string)?;
        let last_state: DockerManagerResponse = serde_json::from_str(&*last_move.game_state).unwrap();

        if last_state.requested_actions.len() == 1 {

            self.communicate(pool, connections, user, actions_request, lobby_members, lobby).await?;
            return Ok(());
        }

        let actions = connections.get_from_game_move_history_id(last_move.id).await;
        let mut new_actions;

        if actions == "" {
            new_actions = actions_request;
        } else {
            new_actions = serde_json::from_str(&*actions).unwrap();
            new_actions.add_action_obj(actions_request.actions[0].clone());
        }

        if new_actions.actions.len() == last_state.requested_actions.len() {
            self.communicate(pool, connections, user, new_actions, lobby_members, lobby).await?;
        } else {
            connections.set_to_game_move_history_id(last_move.id, serde_json::to_string(&new_actions).unwrap());
        }

        return Ok(());
    }

    pub async fn communicate(&self, pool: ConnectionPool, connections: Extension<Connections>, user: User, actions_request: ActionsRequest, lobby_members: Vec<LobbyMember>, lobby: Lobby) -> Result<(), String> {

        let docker_manager_service = DockerManagerService::new(pool.clone());
        let lobby_service = LobbyService::new(pool.clone());

        let docker_manager_response = docker_manager_service.communicate_docker_manager(user.id, serde_json::to_string(&actions_request).unwrap()).await.map_err(status_code_to_string)?;
        if docker_manager_response.game_state.game_over {
            let mut winner_index = 1;
            let mut max_score = docker_manager_response.game_state.scores[0];
            let mut loser_rankings = 0;
            let mut nb_losers = 0;
            for i in 1..docker_manager_response.game_state.scores.len() {
                if max_score < docker_manager_response.game_state.scores[i] {
                    max_score = docker_manager_response.game_state.scores[i];
                    winner_index = i+1;
                }
            }
            println!("winner_index : {:?} // max_score : {:?}",winner_index,max_score);

            let mut losers_id = vec![];
            for lobby_member in &lobby_members {
                if lobby_member.player != winner_index as i32 {
                    loser_rankings += docker_manager_service.get_ranking(lobby_member.user_id, lobby.game_id).await.map_err(status_code_to_string)?;
                    nb_losers += 1;
                    losers_id.push(lobby_member.user_id);
                }
                println!("total ranking : {:?} // nblosers : {:?} // losers_id : {:?}",loser_rankings,nb_losers,losers_id);
                lobby_service.exit_lobby(lobby_member.user_id).await.map_err(status_code_to_string)?;
                docker_manager_service.add_experience(lobby_member.user_id).await.map_err(status_code_to_string)?;
            }
            docker_manager_service.update_ranking(lobby_members[winner_index-1].user_id,lobby.game_id,losers_id,false,loser_rankings/nb_losers).await.map_err(status_code_to_string)?;

        }

        docker_manager_response.send_to_users(connections, lobby_members).await;

        return Ok(());
    }
}