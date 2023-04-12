use serde::Serialize;
use crate::entrypoint::websocket::response::message_response::MessageResponse;

#[derive(Serialize, Debug)]
pub enum ResponseEnum {
    Hello,
    Pong,
    Bye,
    Message(MessageResponse),
    BadMessage,
}
