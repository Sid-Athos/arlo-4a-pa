use axum::Extension;
use serde::Deserialize;
use crate::domain::model::user::User;
use crate::entrypoint::websocket::connections::Connections;
use crate::entrypoint::websocket::request::message_request::MessageRequest;
use crate::entrypoint::websocket::response::message_response::MessageResponse;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;

#[derive(Deserialize, Debug)]
pub enum RequestEnum {
    Hello,
    Ping,
    Exit,
    Message(MessageRequest),
    BadMessage,
}

impl RequestEnum {

    pub async fn compute(&self, connections: Extension<Connections>, user: User) -> bool {
        match self {
            RequestEnum::Hello => {
                connections.send_to_vec_user_id(ResponseEnum::Hello, vec![user.id]).await;
                false
            }
            RequestEnum::Ping => {
                connections.send_to_vec_user_id(ResponseEnum::Pong, vec![user.id]).await;
                false
            }
            RequestEnum::Exit => {
                connections.send_to_vec_user_id(ResponseEnum::Bye, vec![user.id]).await;
                true
            }
            RequestEnum::Message(message) => {
                let response = MessageResponse {
                    from_user: user.id,
                    message: message.message.clone(),
                };
                connections.send_to_vec_user_id(ResponseEnum::Message(response), vec![message.to_user]).await;
                false
            }
            RequestEnum::BadMessage => {
                connections.send_to_vec_user_id(ResponseEnum::BadMessage, vec![user.id]).await;
                false
            }
        }
    }
}
