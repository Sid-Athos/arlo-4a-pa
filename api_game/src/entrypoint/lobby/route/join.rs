use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::model::user::User;
use crate::entrypoint::lobby::route::response::lobby_member_response::LobbyMemberResponse;
use crate::service::lobby_service::LobbyService;

pub async fn join_lobby(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, Path(lobby_id): Path<i32>) -> Result<Json<LobbyMemberResponse>, StatusCode> {

    let lobby_service = LobbyService::new(
        GameRepository::new(pool.clone()),
        LobbyRepository::new(pool.clone()),
        LobbyMemberRepository::new(pool.clone()),
    );

    let lobby_member = lobby_service.join_lobby(user.id, lobby_id).await?;

    Ok(Json(LobbyMemberResponse::from_domain(user, lobby_member)))
}