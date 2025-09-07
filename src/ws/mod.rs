use futures_util::StreamExt;
use serde_json::json;
use warp::{filters::ws::WebSocket, Filter};

use crate::cache::{WebSocketUser, CONNECTED};
use crate::db::establish_connection;
use crate::models::User;
use crate::middleware::with_user;
use crate::ws::messages::WsMessage;
mod messages;
mod types;
mod manager;
mod util;

async fn chat(socket: WebSocket, user: User) {
    tokio::spawn(async move {
        let (tx,mut rx) = socket.split();
        let mut connected = CONNECTED.lock().await;
        let u = WebSocketUser::new(tx);
        connected.insert(user.id, u.clone());
        let conn = &mut establish_connection();
        while let Some(msg) = rx.next().await {
            match msg {
                Ok(msg) => {
                    let msg = if let Ok(msg) = msg.to_str() {
                        msg
                    } else {
                        continue;
                    };
                    let payload = if let Ok(p) = serde_json::from_str::<WsMessage>(msg) {
                        p
                    } else {
                        let _ = u.send(json!({
                            "id": "",
                            "type": "ERROR",
                            "message": "INVALID PAYLOAD"
                        }).to_string()).await;
                        continue;
                    };
                    match manager::handle_message(payload, &user, conn).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error handling message: {}", e);
                        }
                    };
                },
                Err(e) => {
                    println!("Error receiving message: {}", e);
                    break;
                }
            }
        }
        connected.remove(&user.id);
    });
}

pub fn chat_handler() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("chat")
        .and(warp::ws())
        .and(with_user())
        .map(|ws: warp::ws::Ws, user: User| 
            ws.on_upgrade(move |socket| chat(socket,user))
        )
}