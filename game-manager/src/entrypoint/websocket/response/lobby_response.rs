use axum::Extension;
use serde::Serialize;
use crate::database::init::ConnectionPool;
use crate::domain::model::lobby::Lobby;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::lobby_member_response::LobbyMemberResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;

#[derive(Serialize, Debug)]
pub struct LobbyResponse {
    pub id: i32,
    pub code: String,
    pub game_id: i32,
    pub private: bool,
    pub members: Vec<LobbyMemberResponse>,
}

impl LobbyResponse {

    pub fn from_domain(lobby: Lobby, lobby_members: Vec<LobbyMemberResponse>) -> Self {
        LobbyResponse {
            id: lobby.id,
            code: lobby.code,
            game_id: lobby.game_id,
            private: lobby.private,
            members: lobby_members,
        }
    }

    pub(crate) async fn send_lobby_to_members(pool: ConnectionPool, connections: Extension<Connections>, user: User) -> Result<(), String> {
        let lobby_service = LobbyService::new(pool.clone());
        let user_service = UserService::new(pool);

        let lobby = lobby_service.get_by_user_id(user.id).await?;

        let members = lobby_service.get_lobby_member(lobby.id).await?;

        let mut lobby_members_response = Vec::new();
        let mut members_user_id = Vec::new();

        for member in members {
            members_user_id.push(member.user_id);
            let user = user_service.get_user_by_id(member.user_id).await?;
            lobby_members_response.push(LobbyMemberResponse::from_domain(user, member));
        }

        let lobby_response = LobbyResponse::from_domain(lobby, lobby_members_response);

        connections.send_to_vec_user_id(ResponseEnum::Lobby(lobby_response), members_user_id).await;

        Ok(())
    }
}
