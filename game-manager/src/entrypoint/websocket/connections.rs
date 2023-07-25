use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::ws::{Message, WebSocket};
use futures_util::{AsyncWriteExt, SinkExt};
use futures_util::stream::SplitSink;
use tokio::sync::Mutex;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;

#[derive(Debug, Clone)]
pub struct Connections {
    clients: Arc<Mutex<HashMap<i32, SplitSink<WebSocket, Message>>>>,
    lobbies_actions: Arc<Mutex<HashMap<i32, String>>>
}

impl Connections {
    pub(crate) fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            lobbies_actions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn get_from_game_move_history_id(&self, game_move_history_id: i32) -> String {
        for (i, val) in self.lobbies_actions.lock().await.iter_mut() {
            if *i == game_move_history_id {
                return val.clone();
            }
        }

        return String::new();
    }

    pub async fn set_to_game_move_history_id(&self, game_move_history_id: i32, actions: String) {
        if self.lobbies_actions.lock().await.contains_key(&game_move_history_id) {
            self.lobbies_actions.lock().await.remove(&game_move_history_id);
        }
        self.lobbies_actions.lock().await.insert(game_move_history_id, actions);
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