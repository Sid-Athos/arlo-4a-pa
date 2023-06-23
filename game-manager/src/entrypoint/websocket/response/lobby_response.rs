use axum::Extension;

use serde::Serialize;
use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::lobby::Lobby;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::game_response::GameResponse;
use crate::entrypoint::websocket::response::lobby_member_response::LobbyMemberResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::game_service::GameService;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;

#[derive(Serialize, Debug, Clone)]
pub struct LobbyResponse {
    pub id: i32,
    pub code: String,
    pub game_id: i32,
    pub private: bool,
    pub members: Vec<LobbyMemberResponse>,
    pub game: GameResponse,
}

impl LobbyResponse {

    pub async fn from_domain(lobby: Lobby, pool: ConnectionPool) -> Result<Self, String> {

        let lobby_service = LobbyService::new(pool.clone());
        let game_service = GameService::new(pool.clone());
        let user_service = UserService::new(pool);

        let game = game_service.get_by_id(lobby.game_id).await.map_err(status_code_to_string)?;
        let members = lobby_service.get_lobby_member(lobby.id).await.map_err(status_code_to_string)?;

        let mut lobby_members_response = Vec::new();

        for member in members {
            let user = user_service.get_user_by_id(member.user_id).await.map_err(status_code_to_string)?;
            lobby_members_response.push(LobbyMemberResponse::from_domain(user, member));
        }

        Ok(LobbyResponse {
            id: lobby.id,
            code: lobby.code,
            game_id: lobby.game_id,
            private: lobby.private,
            members: lobby_members_response,
            game: GameResponse::from_domain(game),
        })
    }

    pub async fn send_lobby_to_members_from_lobby_id(pool: ConnectionPool, connections: Extension<Connections>, lobby_id: i32) -> Result<(), String> {

        let lobby_service = LobbyService::new(pool.clone());
        let lobby = lobby_service.get_by_id(lobby_id).await.map_err(status_code_to_string)?;

        let lobby_response = LobbyResponse::from_domain(lobby, pool).await?;

        let mut members_user_id = Vec::new();
        for member in &lobby_response.members {
            members_user_id.push(member.id);
        }

        connections.send_to_vec_user_id(ResponseEnum::Lobby(lobby_response), members_user_id).await;

        Ok(())
    }

    pub async fn send_lobby_to_members(pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool.clone());
        let lobby = lobby_service.get_by_user_id(user.id).await.map_err(status_code_to_string)?;

        let lobby_response = LobbyResponse::from_domain(lobby, pool).await?;

        let mut members_user_id = Vec::new();
        for member in &lobby_response.members {
            members_user_id.push(member.id);
        }

        connections.send_to_vec_user_id(ResponseEnum::Lobby(lobby_response), members_user_id).await;

        Ok(())
    }
}
