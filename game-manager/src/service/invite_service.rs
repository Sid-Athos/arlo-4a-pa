use crate::database::init::ConnectionPool;
use crate::database::repository::invite_repository::InviteRepository;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::error::database_error_to_response_error;
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

    pub async fn invite_user_id(&self, from_user_id: i32, to_user_id: i32) -> Result<Invite, String> {
        let lobby_member = self.lobby_member_repository.get_by_user_id(from_user_id).await.map_err(database_error_to_response_error)?;
        let lobby = self.lobby_repository.get_by_id(lobby_member.lobby_id).await.map_err(database_error_to_response_error)?;
        self.invite_repository.create(lobby.id, from_user_id, to_user_id).await.map_err(database_error_to_response_error)
    }

    pub async fn cancel_invite(&self, from_user_id: i32, to_user_id: i32) -> Result<Invite, String> {
        let lobby_member = self.lobby_member_repository.get_by_user_id(from_user_id).await.map_err(database_error_to_response_error)?;
        let lobby = self.lobby_repository.get_by_id(lobby_member.lobby_id).await.map_err(database_error_to_response_error)?;
        let invite = self.invite_repository.get_by_users_id_and_lobby_id(from_user_id, to_user_id, lobby.id).await.map_err(database_error_to_response_error)?;
        self.invite_repository.delete_by_id(invite.id).await.map_err(database_error_to_response_error)
    }
}