use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::model::user::User;
use crate::entrypoint::lobby::route::request::create_request::CreateLobbyRequest;
use crate::entrypoint::lobby::route::response::lobby_response::LobbyResponse;
use crate::service::command::create_lobby_command::CreateLobbyCommand;
use crate::service::lobby_service::LobbyService;


pub async fn create_lobby(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, Json(create_lobby_request): Json<CreateLobbyRequest>) -> Result<Json<LobbyResponse>, StatusCode> {

    let lobby_service = LobbyService::new(
        LobbyRepository::new(pool.clone()),
        LobbyMemberRepository::new(pool.clone()),
    );

    let command = CreateLobbyCommand::new(user.id, create_lobby_request.game_id, create_lobby_request.private);

    let lobby = lobby_service.create(command).await?;

    Ok(Json(LobbyResponse::from_domain(lobby)))
}