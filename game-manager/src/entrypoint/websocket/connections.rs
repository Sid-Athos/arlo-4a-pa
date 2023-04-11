use std::sync::Arc;
use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Connections {
    clients: Arc<Mutex<Vec<SplitSink<WebSocket, Message>>>>,
}

impl Connections {
    pub(crate) fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn send_to_all(&self, message: Message) {
        let mut clients = self.clients.lock().await;
        println!("clients: {:?}", clients);
        let mut disconnected_clients = Vec::new();

        for (i, client) in clients.iter_mut().enumerate() {
            if client.send(message.clone()).await.is_err() {
                disconnected_clients.push(i);
            }
        }

        for i in disconnected_clients.into_iter().rev() {
            clients.remove(i);
        }
    }

    pub async fn add_client(&self, client: SplitSink<WebSocket, Message>) {
        self.clients.lock().await.push(client);
    }
}