use once_cell::sync::Lazy;
use serde::Serialize;
use tokio::sync::Mutex;
use uuid::Uuid;
use warp::filters::ws::{Message, WebSocket};
use std::{collections::HashMap, sync::Arc};
use futures_util::{stream::SplitSink, SinkExt};

use crate::result::{CarbonError, CarbonResult};

#[derive(Clone,)]
pub struct WebSocketUser {
    pub socket: Arc<Mutex<Option<SplitSink<WebSocket, Message>>>>,
}

impl WebSocketUser {
    pub fn new(tx: SplitSink<WebSocket,Message>) -> Self {
        Self {
            socket: Arc::new(Mutex::new(Some(tx))),
        }
    }
    
    pub async fn send<S>(&self, payload: S) -> CarbonResult<()> 
    where S: Serialize + Into<String> {
        if let Some(sock) = self.socket.lock().await.as_mut() {
            sock.send(Message::text(payload))
                .await
                .map_err(|_| CarbonError::WebSocketError)
        } else {
            Ok(())
        }
    }
}

pub static CONNECTED: Lazy<Arc<Mutex<HashMap<Uuid,WebSocketUser>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});