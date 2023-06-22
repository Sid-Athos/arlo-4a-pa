use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::error::database_error_to_status_code;
use crate::entrypoint::lobby::route::response::game_response::GameResponse;
use crate::entrypoint::lobby::route::response::lobby_member_response::LobbyMemberResponse;
use crate::entrypoint::lobby::route::response::lobby_response::LobbyResponse;
use crate::service::game_service::GameService;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;

pub async fn get_public_lobby_for_game(State(pool): State<ConnectionPool>, Path(game_id): Path<i32>) -> Result<Json<Vec<LobbyResponse>>, StatusCode> {

    let lobby_service = LobbyService::new(pool.clone());
    let game_service = GameService::new(pool.clone());
    let user_service = UserService::new(pool);

    let lobbies = lobby_service.get_public_by_game_id(game_id).await?;

    let mut result = Vec::new();

    for lobby in lobbies {
        let lobby_members = lobby_service.get_lobby_member(lobby.id).await?;
        let game = game_service.get_by_id(lobby.game_id).await?;

        let mut lobby_members_response = Vec::new();
        for lobby_member in lobby_members {
            let user = user_service.get_user_by_id(lobby_member.user_id).await?;
            let lobby_member_response = LobbyMemberResponse::from_domain(user, lobby_member);
            lobby_members_response.push(lobby_member_response);
        }

        result.push(LobbyResponse::from_domain(lobby, lobby_members_response, GameResponse::from_domain(game)));
    }

    Ok(Json(result))
}