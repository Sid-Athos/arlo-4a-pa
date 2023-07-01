use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use tokio::sync::Mutex;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;

#[derive(Debug, Clone)]
pub struct Connections {
    clients: Arc<Mutex<HashMap<i32, SplitSink<WebSocket, Message>>>>,
}

impl Connections {
    pub(crate) fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get_list_connected(&self) -> Vec<i32> {
        let mut list = Vec::new();

        for (i, _) in self.clients.lock().await.iter_mut() {
            list.push(*i);
        }

        list
    }

    pub async fn send_to_vec_user_id(&self, response: ResponseEnum, users_id: Vec<i32>) {
        let mut disconnected_clients = Vec::new();

        let message = match serde_json::to_string(&response) {
            Ok(str) => str,
            Err(_) => "".to_string()
        };

        for (i, client) in self.clients.lock().await.iter_mut() {
            if users_id.contains(i) {
                if client.send(Message::Text(message.clone())).await.is_err() {
                    disconnected_clients.push(*i);
                }
            }
        }

        for i in disconnected_clients {
            self.clients.lock().await.remove(&i);
        }
    }

    pub async fn add_client(&self, user_id: i32, client: SplitSink<WebSocket, Message>) {
        self.disconnect_client(user_id).await;
        self.clients.lock().await.insert(user_id, client);
    }

    pub async fn disconnect_client(&self, user_id: i32) {
        if self.clients.lock().await.contains_key(&user_id) {
            let c = self.clients.lock().await.remove(&user_id);
            c.unwrap().close().await.unwrap();
        }
    }
}