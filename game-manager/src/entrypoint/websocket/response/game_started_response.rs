use crate::database::init::ConnectionPool;
use crate::domain::error::status_code_to_string;
use crate::domain::model::lobby::Lobby;
use crate::entrypoint::websocket::response::game_response::GameResponse;
use crate::entrypoint::websocket::response::lobby_member_response::LobbyMemberResponse;
use crate::service::game_service::GameService;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GameStartedResponse {
    pub id: i32,
    pub members: Vec<LobbyMemberResponse>,
    pub game: GameResponse,
}

impl GameStartedResponse {

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

        Ok(GameStartedResponse {
            id: lobby.id,
            members: lobby_members_response,
            game: GameResponse::from_domain(game),
        })
    }
}
