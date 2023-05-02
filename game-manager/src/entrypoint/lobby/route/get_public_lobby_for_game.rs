use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::entrypoint::lobby::route::response::lobby_response::LobbyResponse;
use crate::service::lobby_service::LobbyService;

pub async fn get_public_lobby_for_game(State(pool): State<ConnectionPool>, Path(game_id): Path<i32>) -> Result<Json<Vec<LobbyResponse>>, StatusCode> {

    let lobby_service = LobbyService::new(pool);

    let lobbies = lobby_service.get_public_by_game_id(game_id).await.unwrap();

    Ok(Json(LobbyResponse::from_vec_domain(lobbies)))
}