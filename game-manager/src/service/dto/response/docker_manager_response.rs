use axum::Extension;
use serde::{Deserialize, Serialize};
use crate::domain::model::lobby_member::LobbyMember;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::game_actions_response::GameActionsResponse;
use crate::entrypoint::websocket::response::game_display_response::GameDisplayResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::dto::response::actions_response::ActionsResponse;
use crate::service::dto::response::display_response::DisplayResponse;
use crate::service::dto::response::game_state_response::GameStateResponse;

#[derive(Serialize, Deserialize)]
pub struct DockerManagerResponse {
    pub displays: Vec<DisplayResponse>,
    pub requested_actions: Vec<ActionsResponse>,
    pub game_state: GameStateResponse
}

impl DockerManagerResponse {

    pub async fn send_to_users(&self, connections: Extension<Connections>, lobby_members: Vec<LobbyMember>) {

        for display in &self.displays {
            let game_display_response = GameDisplayResponse::new(display.width.clone(), display.height.clone(), display.content.clone());
            let mut send_to = vec![];

            for lobby_member in &lobby_members {
                if lobby_member.player == display.player {
                    send_to.push(lobby_member.user_id);
                }
            }

            connections.send_to_vec_user_id(ResponseEnum::GameDisplay(game_display_response), send_to).await;
        }

        for requested_action in &self.requested_actions {
            let game_action_response = GameActionsResponse::new(requested_action.player.clone(), requested_action._type.clone(), requested_action.zones.clone());
            let mut send_to = vec![];

            for lobby_member in &lobby_members {
                if lobby_member.player == requested_action.player {
                    send_to.push(lobby_member.user_id);
                }
            }

            connections.send_to_vec_user_id(ResponseEnum::GameAction(game_action_response), send_to).await;
        }

        if self.game_state.game_over {
            let mut winners = vec![];
            let mut send_to_winners = vec![];
            let mut send_to_losers = vec![];

            for i in 0..self.game_state.scores.len() {
                if self.game_state.scores[i] == 1 {
                    winners.push((i + 1) as i32);
                }
            }

            for lobby_member in &lobby_members {
                if winners.contains(&lobby_member.player) {
                    send_to_winners.push(lobby_member.user_id);
                } else {
                    send_to_losers.push(lobby_member.user_id);
                }
            }

            connections.send_to_vec_user_id(ResponseEnum::GameWin, send_to_winners).await;
            connections.send_to_vec_user_id(ResponseEnum::GameLose, send_to_losers).await;
        }

    }
}