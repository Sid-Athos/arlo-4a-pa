use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use tokio::sync::Mutex;
use crate::entrypoint::websocket::response::response_enum::ResponseEnum;

#[derive(Debug, Clone)]
pub struct ConnectionsTest {
    clients: Arc<Mutex<Vec<SplitSink<WebSocket, Message>>>>,
}

impl ConnectionsTest {
    pub(crate) fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn send_to_all(&self, response: Message) {
        for client in self.clients.lock().await.iter_mut() {
            client.send(response.clone()).await;
        }
    }

    pub async fn add_client(&self, client: SplitSink<WebSocket, Message>) {
        self.clients.lock().await.push(client);
    }
}