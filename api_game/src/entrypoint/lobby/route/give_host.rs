use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::database::repository::session_repository::SessionRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::domain::model::lobby_member::LobbyMember;
use crate::domain::model::user::User;
use crate::entrypoint::lobby::route::response::lobby_member_response::LobbyMemberResponse;
use crate::service::lobby_service::LobbyService;
use crate::service::user_service::UserService;

pub async fn give_host(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, Extension(lobby_member): Extension<LobbyMember>, Path(user_id): Path<i32>) -> Result<Json<LobbyMemberResponse>, StatusCode> {

    let lobby_service = LobbyService::new(
        GameRepository::new(pool.clone()),
        LobbyRepository::new(pool.clone()),
        LobbyMemberRepository::new(pool.clone()),
    );

    let user_service = UserService::new(
        UserRepository::new(pool.clone()),
        SessionRepository::new(pool.clone()),
    );

    let user_to_give_host = user_service.get_user_by_id(user_id).await?;

    let lobby_member = lobby_service.give_host(user.id, user_id, lobby_member.lobby_id).await?;

    Ok(Json(LobbyMemberResponse::from_domain(user_to_give_host, lobby_member)))
}
