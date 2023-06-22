use axum::http::StatusCode;
use crate::database::database_error::DatabaseError;
use crate::database::init::ConnectionPool;
use crate::database::repository::invite_repository::InviteRepository;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::error::{database_error_to_response_error, database_error_to_status_code};
use crate::domain::model::invite::Invite;

pub struct InviteService {
    pub lobby_repository: LobbyRepository,
    pub invite_repository: InviteRepository,
    pub lobby_member_repository: LobbyMemberRepository,
}

impl InviteService {

    pub fn new(pool: ConnectionPool) -> Self {
        InviteService {
            lobby_repository: LobbyRepository::new(pool.clone()),
            invite_repository: InviteRepository::new(pool.clone()),
            lobby_member_repository: LobbyMemberRepository::new(pool.clone()),
        }
    }

    pub async fn create_invite(&self, from_user_id: i32, to_user_id: i32) -> Result<Invite, StatusCode> {
        let lobby_member = self.lobby_member_repository.get_by_user_id(from_user_id).await.map_err(database_error_to_status_code)?;
        let lobby = self.lobby_repository.get_by_id(lobby_member.lobby_id).await.map_err(database_error_to_status_code)?;
        let invite = self.invite_repository.get_by_users_id_and_lobby_id(from_user_id, to_user_id, lobby.id).await;
        match invite {
            Ok(i) => Ok(i),
            Err(e) => {
                match e {
                    DatabaseError::NotFound => {
                        self.invite_repository.create(lobby.id, from_user_id, to_user_id).await.map_err(database_error_to_status_code)
                    },
                    _ => Err(database_error_to_status_code(e))
                }
            }
        }
    }

    pub async fn cancel_invite(&self, from_user_id: i32, to_user_id: i32) -> Result<Invite, StatusCode> {
        let lobby_member = self.lobby_member_repository.get_by_user_id(from_user_id).await.map_err(database_error_to_status_code)?;
        let lobby = self.lobby_repository.get_by_id(lobby_member.lobby_id).await.map_err(database_error_to_status_code)?;
        let invite = self.invite_repository.get_by_users_id_and_lobby_id(from_user_id, to_user_id, lobby.id).await.map_err(database_error_to_status_code)?;
        self.invite_repository.delete_by_id(invite.id).await.map_err(database_error_to_status_code)
    }

    pub async fn accept_invite(&self, from_user_id: i32, to_user_id: i32) -> Result<Invite, StatusCode> {
        let lobby_member = self.lobby_member_repository.get_by_user_id(from_user_id).await.map_err(database_error_to_status_code)?;
        let lobby = self.lobby_repository.get_by_id(lobby_member.lobby_id).await.map_err(database_error_to_status_code)?;
        let invite = self.invite_repository.get_by_users_id_and_lobby_id(from_user_id, to_user_id, lobby.id).await.map_err(database_error_to_status_code)?;
        self.invite_repository.delete_by_id(invite.id).await.map_err(database_error_to_status_code)?;
        self.lobby_member_repository.create(to_user_id, lobby.id, false).await.map_err(database_error_to_status_code)?;
        Ok(invite)
    }

    pub async fn cancel_all_from_user_id(&self, from_user_id: i32) -> Result<Vec<Invite>, StatusCode> {
        self.invite_repository.delete_all_from_user_id(from_user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_invites_sent_by_user_id(&self, user_id: i32) -> Result<Vec<Invite>, StatusCode> {
        self.invite_repository.get_invites_from_by_user_id(user_id).await.map_err(database_error_to_status_code)
    }
}
