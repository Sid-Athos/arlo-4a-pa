use axum::{Extension, Json};
use axum::extract::State;
use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::model::user::User;
use crate::entrypoint::lobby::route::response::lobby_response::LobbyResponse;
use crate::service::lobby_service::LobbyService;

pub async fn me(State(pool): State<ConnectionPool>, Extension(user): Extension<User>) -> Result<Json<LobbyResponse>, StatusCode> {

    let lobby_service = LobbyService::new(
        GameRepository::new(pool.clone()),
        LobbyRepository::new(pool.clone()),
        LobbyMemberRepository::new(pool.clone()),
    );

    let lobby = lobby_service.get_by_user_id(user.id).await?;

    Ok(Json(LobbyResponse::from_domain(lobby)))
}