use chrono::Utc;
use diesel::{PgConnection,RunQueryDsl,QueryDsl,ExpressionMethods};
use uuid::Uuid;
use serde_json::json;

use crate::{cache::CONNECTED, models::{Chat, ChatMember, Message, User}, result::CarbonResult, ws::{messages::WsMessage, util::{get_usernames, parse_mentions}}};

pub async fn handle_message(msg: WsMessage, user: &User, conn: &mut PgConnection) -> CarbonResult<()> {
    match msg {
        WsMessage::SendMessage { id, payload } => {
            let message = Message {
                id: Uuid::new_v4(),
                mentions: parse_mentions(payload.content.as_ref(), conn)?,
                content: payload.content,
                media: payload.media,
                sent_at: Utc::now().naive_utc(),
                author: user.id,
                chat_id: payload.chat_id
            };
            {
                use crate::schema::messages;
                diesel::insert_into(messages::table)
                    .values(&message)
                    .execute(conn)?;
            }
            let targets = {
                use crate::schema::chat_member::dsl::*;
                chat_member.filter(chat_id.eq(payload.chat_id)).select(user_id).load::<Uuid>(conn)?
            };
            let connected = CONNECTED.lock().await;
            for connection in connected.iter() {
                if targets.contains(connection.0) && connection.0 != &user.id {
                    let _ = connection.1.send(json!({
                        "id": id.to_string(),
                        "type": "NEW_MESSAGE",
                        "data": message
                    }).to_string()).await;
                }
            };
        },
        WsMessage::CreateChat { id, payload } => {
            if payload.symkeys.len() != payload.members.len() {
                return Err(crate::result::CarbonError::UserError { message: "Invalid payload".to_string(), code: 400 })
            }
            let chat = {
                use crate::schema::chats;
                diesel::insert_into(chats::table)
                    .values(Chat {
                        id: Uuid::new_v4(),
                        name: payload.name.unwrap_or(get_usernames(&payload.members, conn)?.join(", ")),
                        broadcast: payload.broadcast,
                        img_src: payload.img_src,
                        description: payload.description,
                        admins: payload.admins
                    })
                    .get_result::<Chat>(conn)?
            };
            let members = {
                use crate::schema::chat_member;
                diesel::insert_into(chat_member::table)
                    .values(&payload.members.iter().map(|m| ChatMember {
                        chat_id: chat.id,
                        invited_by: Some(user.id),
                        user_id: *m,
                        joined_at: Utc::now().naive_utc(),
                        symkey: payload.symkeys.get(m).cloned()
                    }).collect::<Vec<ChatMember>>())
                    .execute(conn)?;
                payload.members
            };
            let connected = CONNECTED.lock().await;
            for connection in connected.iter() {
                if members.contains(connection.0) && connection.0 != &user.id {
                    let _ = connection.1.send(json!({
                        "id": id.to_string(),
                        "type": "NEW_CHAT",
                        "data": { "chat": chat, "symkey": &payload.symkeys.get(connection.0) }
                    }).to_string()).await;
                }
            };
        },
        WsMessage::InviteUser { id, payload } => {
            {
                use crate::schema::chat_member;
                diesel::insert_into(chat_member::table)
                    .values(&ChatMember {
                        chat_id: payload.chat_id,
                        invited_by: Some(user.id),
                        user_id: payload.user_id,
                        joined_at: Utc::now().naive_utc(),
                        symkey: Some(payload.symkey.clone())
                    })
                    .execute(conn)?;
            };
            let members = {
                use crate::schema::chat_member;
                chat_member::table
                    .filter(chat_member::chat_id.eq(payload.chat_id))
                    .select(chat_member::user_id)
                    .load::<Uuid>(conn)?
            };
            let chat = {
                use crate::schema::chats;
                chats::table
                    .filter(chats::id.eq(payload.chat_id))
                    .first::<Chat>(conn)?
            };
            let connected = CONNECTED.lock().await;
            for connection in connected.iter() {
                if members.contains(connection.0) 
                && connection.0 != &user.id 
                && connection.0 != &payload.user_id {
                    let _ = connection.1.send(json!({
                        "id": id.to_string(),
                        "type": "NEW_USER",
                        "data": payload.user_id.to_string()
                    }).to_string()).await;
                } else if connection.0 == &payload.user_id {
                    let _ = connection.1.send(json!({
                        "id": id.to_string(),
                        "type": "NEW_CHAT",
                        "data": { "chat": chat, "symkey": payload.symkey }
                    }).to_string()).await;
                }
            };
        }
    };
    Ok(())
}