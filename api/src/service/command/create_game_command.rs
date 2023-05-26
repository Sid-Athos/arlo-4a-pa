use crate::entrypoint::game::route::request::create_game_request::CreateGameRequest;
use crate::entrypoint::user::route::request::create_user_request::CreateUserRequest;

#[derive(Debug)]
pub struct CreateGameCommand {
    pub name: String,
    pub min_players : i32,
    pub max_players : i32,
    pub description : String,
    pub path : String
}

impl CreateGameCommand {

    pub fn new(request: CreateGameRequest) -> Self {
        CreateGameCommand {
            name: request.name,
            min_players: request.min_players,
            max_players: request.max_players,
            description: request.description,
            path: request.path
        }
    }
}