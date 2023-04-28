use axum::Extension;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;

pub struct ErrorResponse {}

impl ErrorResponse {

    pub(crate) async fn send_error(result: Result<(), String>, connections: Extension<Connections>, user: User) {
        if result.is_err() {
            connections.send_to_vec_user_id(ResponseEnum::Error(result.err().unwrap_or("Failed when getting error".to_string())), vec![user.id]).await;
        }
    }
}