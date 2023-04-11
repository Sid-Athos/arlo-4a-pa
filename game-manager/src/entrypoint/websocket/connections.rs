use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use tokio::sync::Mutex;

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

    pub async fn send_to_all(&self, message: Message) {
        let mut clients = self.clients.lock().await;
        let mut disconnected_clients = Vec::new();

        for (i, client) in clients.iter_mut() {
            if client.send(message.clone()).await.is_err() {
                disconnected_clients.push(i);
            }
        }
    }

    pub async fn add_client(&self, user_id: i32, client: SplitSink<WebSocket, Message>) {
        self.clients.lock().await.insert(user_id, client);
    }
}