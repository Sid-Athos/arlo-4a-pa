use std::fs::File;
use std::ptr::null;
use axum::body::Bytes;
use axum::extract::{State, Multipart};
use axum::http::StatusCode;
use axum::{Extension, Json};
use crate::database::init::ConnectionPool;
use crate::database::repository::game_repository::GameRepository;
use crate::domain::model::user::User;
use crate::entrypoint::game::route::response::game_response::GameResponse;
use crate::service::game_service::GameService;

fn process_bytes(bytes: Bytes) -> String{
    // Convert Bytes to a UTF-8 encoded Vec<u8>
    let utf8_bytes = bytes.to_vec();

    // Convert the UTF-8 encoded bytes to a String
    let text = String::from_utf8_lossy(&utf8_bytes).to_string();

    text
}

pub async fn create_game(State(pool): State<ConnectionPool>, Extension(user): Extension<User>, mut form: Multipart) -> Result<Json<GameResponse>, StatusCode> {

    let game_service = GameService::new(pool.clone());


    let mut name : String = "".to_string();
    let mut max_players : i32 = 0;
    let mut min_players : i32 = 0;
    let mut description : String = "".to_string();
    let mut language : String = "".to_string();
    let mut file : String = "".to_string();

    while let Some(mut field) = form.next_field().await.unwrap() {
        let fieldName = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match &*fieldName {
            "name" => {
                name = process_bytes(data);
            },
            "max_players" => {
                max_players = process_bytes(data).parse::<i32>().unwrap();
            },
            "min_players" => {
                min_players = process_bytes(data).parse::<i32>().unwrap();
            },
            "description" => {
                description = process_bytes(data);
            },
            "tarace" => {
                file = process_bytes(data);
            },
            "language" => {
                language = process_bytes(data);
            },
            _ => {
                return Err(StatusCode::BAD_REQUEST)
            }
        }
    }
    if name == "".to_string() || max_players == 0 || min_players == 0 || description == "".to_string() || language == "".to_string() {
        return Err(StatusCode::BAD_REQUEST)
    }else{
        let game = game_service.create_game(name,max_players,min_players,description,language,file,user.id).await.unwrap();
        Ok(Json(GameResponse::from_domain(game)))

    }

}