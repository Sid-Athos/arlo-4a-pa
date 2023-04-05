use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::entrypoint::lobby::route::response::lobby_response::LobbyResponse;
use crate::service::lobby_service::LobbyService;

pub async fn get_public_lobby(State(pool): State<ConnectionPool>) -> Result<Json<Vec<LobbyResponse>>, StatusCode> {

    let lobby_service = LobbyService::new(
        LobbyRepository::new(pool.clone()),
    );

    let lobbies = lobby_service.get_public().await?;

    Ok(Json(LobbyResponse::from_vec_domain(lobbies)))
}