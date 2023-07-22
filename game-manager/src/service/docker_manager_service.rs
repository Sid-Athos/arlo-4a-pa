use axum::body::HttpBody;
use crate::database::init::ConnectionPool;
use crate::database::repository::game_history_repository::GameHistoryRepository;
use crate::database::repository::game_move_history_repository::GameMoveHistoryRepository;
use crate::database::repository::game_repository::GameRepository;
use crate::database::repository::lobby_repository::LobbyRepository;
use crate::domain::error::database_error_to_status_code;
use crate::domain::model::game_move_history::GameMoveHistory;
use crate::service::dto::game_dto::GameDto;
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

    pub async fn cockmmunicate_with_arnaud_mom(&self, user_id : i32, user_move : String){
        let mut current_lobby = self.lobby_service.get_by_user_id(user_id).await.map_err(database_error_to_status_code)?;

        let game = self.game_repository.get_by_id(current_lobby.game_id).await.map_err(database_error_to_status_code)?;
        let game_id = game.id;
        let game_tag = game.tag;
        let game_language = game.language;
        let lobby_member = self.lobby_service.get_lobby_member_by_user_id(user_id).await.map_err(database_error_to_status_code)?;
        let lobby_members = self.lobby_service.get_lobby_member(current_lobby.id);
        let player = lobby_member.player;
        match current_lobby.game_history_id {
            Some(value) => {
                let current_history = self.game_history_repository.get_by_id(value);
            },
            None => {
                let current_history = self.game_history_repository.create(lobby_members.len() as i32, game_id).await.map_err(database_error_to_status_code)?;
                current_lobby = self.lobby_repository.set_game_history_id(current_lobby.id, current_history.id)
            }
        }

        /*
        let game_dto = GameDto::new(game_language,Vec!());

        let client = reqwest::Client::new();
        let res = client.post(format!("http://dev.mikusupremacy.fr:7588/containers/{}", game_tag))
            .body()
            .send()
            .await?;

        let add_move = self.game_move_history_repository.create()*/

    }


}