use std::collections::HashMap;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use crate::database::init::ConnectionPool;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::entrypoint::lobby::route::response::lobby_response::LobbyResponse;
use crate::service::lobby_service::LobbyService;

pub async fn search(State(pool): State<ConnectionPool>, Query(params): Query<HashMap<String, String>>) -> Result<Json<LobbyResponse>, StatusCode> {

    let lobby_service = LobbyService::new(
        LobbyRepository::new(pool.clone()),
        LobbyMemberRepository::new(pool.clone()),
    );

    let code = params.get("code").ok_or_else(|| StatusCode::BAD_REQUEST)?.to_string();

    let lobby = lobby_service.get_lobby_by_code(code).await?;

    Ok(Json(LobbyResponse::from_domain(lobby)))
}
