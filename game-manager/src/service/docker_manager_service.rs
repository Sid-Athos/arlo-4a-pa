use std::env::current_exe;
use std::str::from_utf8;
use axum::body::HttpBody;
use axum::http::{Method, StatusCode};
use hyper::{Body, Client, Request, Uri};
use hyper::body::{Buf, to_bytes};
use crate::database::init::ConnectionPool;
use crate::database::repository::game_history_repository::GameHistoryRepository;
use crate::database::repository::game_move_history_repository::GameMoveHistoryRepository;
use crate::database::repository::game_repository::GameRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::error::database_error_to_status_code;
use crate::service::dto::command_request::CommandRequest;
use crate::service::dto::docker_manager_response::DockerManagerResponse;
use crate::service::lobby_service::LobbyService;

pub struct DockerManagerService {
    pub game_repository : GameRepository,
    pub game_move_history_repository : GameMoveHistoryRepository,
    pub lobby_service : LobbyService,
    pub game_history_repository : GameHistoryRepository,
    pub lobby_repository : LobbyRepository
}

impl DockerManagerService {
    pub fn new(pool: ConnectionPool) -> Self {
        DockerManagerService {
            game_repository: GameRepository::new(pool.clone()),
            game_move_history_repository : GameMoveHistoryRepository::new(pool.clone()),
            lobby_service : LobbyService::new(pool.clone()),
            game_history_repository : GameHistoryRepository::new(pool.clone()),
            lobby_repository : LobbyRepository::new(pool.clone())
        }
    }

    pub async fn communicate_docker_manager(&self, user_id : i32, user_move : String) -> Result<DockerManagerResponse, StatusCode>{
        let mut current_lobby = self.lobby_service.get_by_user_id(user_id).await?;

        let game = self.game_repository.get_by_id(current_lobby.game_id).await.map_err(database_error_to_status_code)?;
        let lobby_member = self.lobby_service.get_lobby_member_by_user_id(user_id).await?;
        let lobby_members = self.lobby_service.get_lobby_member(current_lobby.id).await?;
        let current_history;

        match current_lobby.game_history_id {
            Some(value) => {
                current_history = self.game_history_repository.get_by_id(value).await.map_err(database_error_to_status_code)?;
            },
            None => {
                current_history = self.game_history_repository.create(lobby_members.len() as i32, game.id).await.map_err(database_error_to_status_code)?;
                current_lobby = self.lobby_repository.set_game_history_id(current_lobby.id, current_history.id).await.map_err(database_error_to_status_code)?;
            }
        }

        let mut moves = self.get_all_past_moves(current_history.id).await?;
        moves.push(user_move.clone());
        let game_dto = CommandRequest::new(game.language, moves);

        let body_str = serde_json::to_string(&game_dto).unwrap();
        println!("body_str: {}", body_str);

        let client = Client::new();
        /*let req = Request::builder()
            .method(Method::POST)
            .uri(format!("http://dev.mikusupremacy.fr:7588/containers/{}", game.tag))
            .body(Body::from(body_str))
            .unwrap();*/
        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("http://dev.mikusupremacy.fr:7588/containers/morpion"))
            .body(Body::from(body_str))
            .unwrap();

        let response = client.request(req).await.unwrap();

        println!("status: {:?}", response.status());
        if response.status() != StatusCode::OK {
            return Err(response.status());
        }
        let bytes = to_bytes(response).await.unwrap();
        let mut data = String::from(from_utf8(&bytes).unwrap());
        data = serde_json::from_str(&*data).unwrap();
        println!("body: {:?}", data);

        self.game_move_history_repository.create(lobby_member.player, data.clone(), user_move, current_history.id).await.map_err(database_error_to_status_code)?;

        return Ok(serde_json::from_str(&*data).unwrap());
    }

    pub async fn get_all_past_moves(&self, history_id: i32) -> Result<Vec<String>, StatusCode> {
        let game_move_history = self.game_move_history_repository.get_all_by_game_history_id(history_id).await.map_err(database_error_to_status_code)?;

        let mut vec_history = vec![];

        for histo in game_move_history {
            vec_history.push(histo.action);
        }

        return Ok(vec_history);
    }
}