use diesel::{table,joinable,allow_tables_to_appear_in_same_query};

table! {
    users (id) {
        id -> Uuid,
        pubkey -> Text,
        username -> Text,
        display_name -> Nullable<Text>,
        img_src -> Nullable<Text>,
        bio -> Nullable<Text>,
        contacts -> Array<Uuid>
    }
}

table! {
    chats (id) {
        id -> Uuid,
        name -> Text,
        broadcast -> Bool,
        img_src -> Nullable<Text>,
        description -> Nullable<Text>,
        admins -> Nullable<Array<Uuid>>
    }
}

table! {
    messages (id) {
        id -> Uuid,
        content -> Nullable<Text>,
        media -> Array<Text>,
        sent_at -> Timestamp,
        mentions -> Array<Uuid>,
        author -> Uuid,
        chat_id -> Uuid,
    }
}

table! {
    message_reaction (message_id) {
        message_id -> Uuid,
        user_id -> Uuid,
        reaction -> Text
    }
}

table! {
    chat_member (chat_id,user_id) {
        chat_id -> Uuid,
        user_id -> Uuid,
        joined_at -> Timestamp,
        invited_by -> Nullable<Uuid>,
        symkey -> Nullable<Text>
    }
}

table! {
    invite (id) {
        id -> Uuid,
        from_user -> Uuid,
        to_user -> Uuid,
        symkey -> Text,
        chat_id -> Uuid
    }
}

joinable!(chat_member -> chats (chat_id));
joinable!(chat_member -> users (user_id));
joinable!(message_reaction -> messages (message_id));
joinable!(messages -> users (author));
joinable!(messages -> chats (chat_id));
joinable!(invite -> users (to_user));
joinable!(invite -> chats (chat_id));

allow_tables_to_appear_in_same_query!(
    users,
    chats,
    chat_member,
    message_reaction,
    messages,
);