use serde::{Serialize,Deserialize};
use uuid::Uuid;

use crate::ws::types::{CreateChatPayload, InviteUserPayload, SendMessagePayload};

#[derive(Serialize,Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WsMessage {
    SendMessage {
        #[serde(with = "uuid::serde::simple")]
        id: Uuid,
        payload: SendMessagePayload,
    },
    InviteUser {
        #[serde(with = "uuid::serde::simple")]
        id: Uuid,
        payload: InviteUserPayload,
    },
    CreateChat {
        #[serde(with = "uuid::serde::simple")]
        id: Uuid,
        payload: CreateChatPayload,
    }
}