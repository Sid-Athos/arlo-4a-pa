use axum::http::StatusCode;
use rand::{Rng, thread_rng};
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::database::repository::lobby_member_repository::LobbyMemberRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::lobby::Lobby;
use crate::domain::model::lobby_member::LobbyMember;
use crate::service::command::create_lobby_command::CreateLobbyCommand;
use crate::service::ws_session_service::WsSessionService;

pub struct LobbyService {
    pub game_repository: GameRepository,
    pub lobby_repository: LobbyRepository,
    pub lobby_member_repository: LobbyMemberRepository,
    pub ws_session_service: WsSessionService,
}

impl LobbyService {

    pub fn new(pool: ConnectionPool) -> Self {
        LobbyService {
            game_repository: GameRepository::new(pool.clone()),
            lobby_repository: LobbyRepository::new(pool.clone()),
            lobby_member_repository: LobbyMemberRepository::new(pool.clone()),
            ws_session_service: WsSessionService::new(pool.clone())
        }
    }

    pub async fn get_by_id(&self, lobby_id: i32) -> Result<Lobby, StatusCode> {
        self.lobby_repository.get_by_id(lobby_id).await.map_err(database_error_to_status_code)
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
            self.ws_session_service.delete_for_lobby(lobby_member.lobby_id).await?;
            return self.lobby_repository.delete_lobby(lobby_member.lobby_id).await.map_err(database_error_to_status_code);
        }
        if lobby_member.is_host {
            let new_host = lobby_members.get(0).unwrap();
            self.lobby_member_repository.update_host(new_host.user_id, lobby_member.lobby_id, true).await.map_err(database_error_to_status_code)?;
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

    pub async fn join_lobby(&self, user_id: i32, lobby_id: i32) -> Result<LobbyMember, StatusCode> {
        let _ = self.exit_lobby(user_id).await;

        let lobby = self.lobby_repository.get_by_id(lobby_id).await.map_err(database_error_to_status_code)?;

        if lobby.private || lobby.is_launched {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let game = self.game_repository.get_by_id(lobby.game_id).await.map_err(database_error_to_status_code)?;

        if game.max_players <= self.lobby_member_repository.get_by_lobby_id(lobby_id).await.map_err(database_error_to_status_code)?.len() as i32 {
            return Err(StatusCode::UNAUTHORIZED);
        }

        self.lobby_member_repository.create(user_id, lobby_id, false).await.map_err(database_error_to_status_code)
    }

    pub async fn get_lobby_member_by_user_id(&self, user_id: i32) -> Result<LobbyMember, StatusCode> {
        self.lobby_member_repository.get_by_user_id(user_id).await.map_err(database_error_to_status_code)
    }

    pub async fn give_host(&self, old_id: i32, next_host: i32, lobby_id: i32) -> Result<LobbyMember, StatusCode> {
        self.lobby_member_repository.update_host(old_id, lobby_id, false).await.map_err(database_error_to_status_code)?;
        self.lobby_member_repository.update_host(next_host, lobby_id, true).await.map_err(database_error_to_status_code)
    }

    pub async fn kick(&self, lobby_id: i32, user_id: i32) -> Result<LobbyMember, StatusCode> {
        self.lobby_member_repository.delete_by_user_id_lobby_id(user_id, lobby_id).await.map_err(database_error_to_status_code)
    }

    pub async fn start_game(&self, lobby_id: i32) -> Result<Lobby, StatusCode> {
        let mut lobby = self.lobby_repository.get_by_id(lobby_id).await.map_err(database_error_to_status_code)?;
        let game = self.game_repository.get_by_id(lobby.game_id).await.map_err(database_error_to_status_code)?;
        let lobby_members = self.lobby_member_repository.get_by_lobby_id(lobby_id).await.map_err(database_error_to_status_code)?;

        if game.min_players > lobby_members.len() as i32 || game.max_players < lobby_members.len() as i32 {
            return Err(StatusCode::NOT_ACCEPTABLE);
        }

        lobby = self.lobby_repository.set_launch_for_lobby_id(lobby_id).await.map_err(database_error_to_status_code)?;
        for i in 0..lobby_members.len() {
            self.lobby_member_repository.set_player(lobby_members[i].id, (i + 1) as i32).await.map_err(database_error_to_status_code)?;
        }

        return Ok(lobby);
    }

    pub async fn set_game_history_id(&self, lobby_id: i32, game_history_id: i32) -> Result<Lobby, StatusCode> {
        self.lobby_repository.set_game_history_id(lobby_id, game_history_id).await.map_err(database_error_to_status_code)
    }
}