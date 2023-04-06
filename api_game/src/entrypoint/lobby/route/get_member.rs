use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::entrypoint::lobby::route::response::lobby_member_response::LobbyMemberResponse;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;

pub async fn get_lobby_member(State(pool): State<ConnectionPool>, Path(lobby_id): Path<i32>) -> Result<Json<Vec<LobbyMemberResponse>>, StatusCode> {

    let lobby_service = LobbyService::new(
        LobbyRepository::new(pool.clone()),
        LobbyMemberRepository::new(pool.clone()),
    );

    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone()),
    );

    let lobby_members = lobby_service.get_lobby_member(lobby_id).await?;
    let mut response = Vec::new();

    for lobby_member in lobby_members {
        let user = user_service.get_user_by_id(lobby_member.user_id).await?;
        response.push(LobbyMemberResponse::from_domain(user, lobby_member));
    }

    if response.is_empty() {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(response))
}