use axum::http::StatusCode;
use rand::{Rng, thread_rng};
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::lobby::Lobby;
use crate::domain::model::lobby_member::LobbyMember;
use crate::service::command::create_lobby_command::CreateLobbyCommand;

pub struct LobbyService {
    pub lobby_repository: LobbyRepository,
    pub lobby_member_repository: LobbyMemberRepository,
}

impl LobbyService {

    pub fn new(lobby_repository: LobbyRepository, lobby_member_repository: LobbyMemberRepository) -> Self {
        LobbyService {
            lobby_repository,
            lobby_member_repository,
        }
    }

    pub async fn get_by_user_id(&self, user_id: i32) -> Result<Lobby, StatusCode> {
        let lobby_member = self.lobby_member_repository.get_by_user_id(user_id).await.map_err(database_error_to_status_code)?;
        self.lobby_repository.get_by_id(lobby_member.lobby_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_public(&self) -> Result<Vec<Lobby>, StatusCode> {
        self.lobby_repository.get_public().await.map_err(database_error_to_status_code)
    }

    pub async fn get_public_by_game_id(&self, game_id: i32) -> Result<Vec<Lobby>, StatusCode> {
        self.lobby_repository.get_public_by_game_id(game_id).await.map_err(database_error_to_status_code)
    }

    pub async fn exit_lobby(&self, user_id: i32) -> Result<Lobby, StatusCode> {
        let lobby_member = self.lobby_member_repository.delete(user_id).await.map_err(database_error_to_status_code)?;
        let lobby_members = self.lobby_member_repository.get_by_lobby_id(lobby_member.lobby_id).await.map_err(database_error_to_status_code)?;

        if lobby_members.len() == 0 {
            return self.lobby_repository.delete_lobby(lobby_member.lobby_id).await.map_err(database_error_to_status_code)
        }
        if lobby_member.is_host {
            let new_host = lobby_members.get(0).unwrap();
            self.lobby_member_repository.update_host(new_host.user_id, true).await.map_err(database_error_to_status_code)?;
        }

        self.lobby_repository.get_by_id(lobby_member.lobby_id).await.map_err(database_error_to_status_code)
    }

    pub async fn get_lobby_member(&self, lobby_id: i32) -> Result<Vec<LobbyMember>, StatusCode> {
        self.lobby_member_repository.get_by_lobby_id(lobby_id).await.map_err(database_error_to_status_code)
    }

    pub async fn create(&self, lobby_command: CreateLobbyCommand) -> Result<Lobby, StatusCode> {

        match self.get_by_user_id(lobby_command.user_id).await {
            Ok(_) => {
                self.exit_lobby(lobby_command.user_id).await?;
            },
            Err(_) => {}
        };

        let mut code = String::new();
        for _ in 0..6 {
            let random_number: u8 = thread_rng().gen_range(0..26) + 65;
            code.push(random_number as char);
        }

        let lobby = self.lobby_repository.create_lobby(code, lobby_command.game_id, lobby_command.private).await.map_err(database_error_to_status_code)?;

        self.lobby_member_repository.create(lobby_command.user_id, lobby.id, true).await.map_err(database_error_to_status_code)?;

        Ok(lobby)
    }

    pub async fn get_lobby_by_code(&self, code: String) -> Result<Lobby, StatusCode> {
        self.lobby_repository.get_by_code(code).await.map_err(database_error_to_status_code)
    }
}