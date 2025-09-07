use std::collections::HashMap;

use serde::{Serialize,Deserialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct SendMessagePayload {
    pub content: Option<String>,
    pub media: Vec<String>,
    #[serde(with = "uuid::serde::simple")]
    pub chat_id: Uuid
}

#[derive(Serialize,Deserialize)]
pub struct CreateChatPayload {
    pub name: Option<String>,
    pub members: Vec<Uuid>,
    pub img_src: Option<String>,
    pub description: Option<String>,
    pub broadcast: bool,
    pub admins: Option<Vec<Uuid>>,
    pub symkeys: HashMap<Uuid,String>
}

#[derive(Serialize,Deserialize)]
pub struct InviteUserPayload {
    #[serde(with = "uuid::serde::simple")]
    pub user_id: Uuid,
    #[serde(with = "uuid::serde::simple")]
    pub chat_id: Uuid,
    pub symkey: String
}

#[derive(Serialize,Deserialize)]
pub struct LeaveChatPayload {
    #[serde(with = "uuid::serde::simple")]
    pub chat_id: Uuid,
}