use axum::http::StatusCode;
use crate::database::init::ConnectionPool;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::database::repository::user_repository::UserRepository;
use crate::database::repository::ws_session_repository::WsSessionRepository;
use crate::domain::error::{database_error_to_status_code};
use crate::domain::model::ws_session::WsSession;

pub struct WsSessionService {
    pub user_repository: UserRepository,
    pub lobby_repository: LobbyRepository,
    pub ws_session_repository: WsSessionRepository,
    pub lobby_member_repository: LobbyMemberRepository,
}

impl WsSessionService {

    pub fn new(pool: ConnectionPool) -> Self {
        WsSessionService {
            user_repository: UserRepository::new(pool.clone()),
            lobby_repository: LobbyRepository::new(pool.clone()),
            ws_session_repository: WsSessionRepository::new(pool.clone()),
            lobby_member_repository: LobbyMemberRepository::new(pool.clone()),
        }
    }

    pub async fn get_by_lobby_id(&self, lobby_id: i32) -> Result<Vec<WsSession>, StatusCode> {
        self.ws_session_repository.get_ws_session_with_lobby_id(lobby_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_by_user_id(&self, user_id: i32) -> Result<WsSession, StatusCode> {
        self.ws_session_repository.get_ws_session_with_user_id(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn create(&self, user_id: i32) -> Result<WsSession, StatusCode> {
        let lobby_member = self.lobby_member_repository.get_by_user_id(user_id).await.map_err(database_error_to_status_code)?;
        let lobby = self.lobby_repository.get_by_id(lobby_member.lobby_id).await.map_err(database_error_to_status_code)?;
        self.ws_session_repository.create(lobby.id, user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn delete_for_lobby(&self, lobby_id: i32) -> Result<Vec<WsSession>, StatusCode> {
        self.ws_session_repository.delete_where_lobby_id(lobby_id).await.map_err(database_error_to_status_code)
    }

    pub async fn delete_for_user(&self, user_id: i32) -> Result<WsSession, StatusCode> {
        self.ws_session_repository.delete_where_user_id(user_id).await.map_err(database_error_to_status_code)
    }
}