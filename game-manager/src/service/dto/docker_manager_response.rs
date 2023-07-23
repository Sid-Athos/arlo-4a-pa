use axum::Extension;
use serde::{Deserialize, Serialize};
use crate::domain::model::lobby_member::LobbyMember;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::game_actions_response::GameActionsResponse;
use crate::entrypoint::websocket::response::game_display_response::GameDisplayResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::dto::actions_response::ActionsResponse;
use crate::service::dto::display_response::DisplayResponse;
use crate::service::dto::game_state_response::GameStateResponse;

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
    }
}