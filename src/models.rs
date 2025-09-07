use diesel::{prelude::{Queryable, QueryableByName}, AsChangeset, Insertable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema;

#[derive(Queryable,QueryableByName,Selectable,AsChangeset,Insertable,Serialize,Deserialize)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: Uuid,
    pub pubkey: String,
    pub username: String,
    pub display_name: Option<String>,
    pub img_src: Option<String>,
    pub bio: Option<String>,
    pub contacts: Vec<Uuid>
}

#[derive(Queryable,QueryableByName,Selectable,AsChangeset,Insertable,Serialize,Deserialize)]
#[diesel(table_name = schema::chats)]
pub struct Chat {
    pub id: Uuid,
    pub name: String,
    pub broadcast: bool,
    pub img_src: Option<String>,
    pub description: Option<String>,
    pub admins: Option<Vec<Uuid>>,
}

#[derive(Queryable,QueryableByName,Selectable,AsChangeset,Insertable,Serialize,Deserialize)]
#[diesel(table_name = schema::messages)]
pub struct Message {
    pub id: Uuid,
    pub content: Option<String>,
    pub media: Vec<String>,
    pub sent_at: NaiveDateTime,
    pub mentions: Vec<Uuid>,
    pub author: Uuid,
    pub chat_id: Uuid
}

#[derive(Queryable,QueryableByName,Selectable,AsChangeset,Insertable,Serialize,Deserialize)]
#[diesel(table_name = schema::message_reaction)]
pub struct MessageReaction {
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub reaction: String
}

#[derive(Queryable,QueryableByName,Selectable,AsChangeset,Insertable,Serialize,Deserialize)]
#[diesel(table_name = schema::chat_member)]
pub struct ChatMember {
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: NaiveDateTime,
    pub invited_by: Option<Uuid>,
    pub symkey: Option<String>,
}

#[derive(Queryable,QueryableByName,Selectable,AsChangeset,Insertable,Serialize,Deserialize)]
#[diesel(table_name = schema::invite)]
pub struct Invite {
    pub id: Uuid,
    pub from_user: Uuid,
    pub to_user: Uuid,
    pub symkey: String,
    pub chat_id: Uuid
}
