// @generated automatically by Diesel CLI.

diesel::table! {
    chat_member (chat_id, user_id) {
        chat_id -> Uuid,
        user_id -> Uuid,
        joined_at -> Timestamp,
        invited_by -> Nullable<Uuid>,
        symkey -> Nullable<Text>,
    }
}

diesel::table! {
    chats (id) {
        id -> Uuid,
        name -> Text,
        broadcast -> Bool,
        img_src -> Nullable<Text>,
        description -> Nullable<Text>,
        admins -> Nullable<Array<Uuid>>,
    }
}

diesel::table! {
    invite (id) {
        id -> Uuid,
        from_user -> Uuid,
        to_user -> Uuid,
        symkey -> Text,
        chat_id -> Uuid,
    }
}

diesel::table! {
    message_reaction (message_id) {
        message_id -> Uuid,
        user_id -> Uuid,
        reaction -> Text,
    }
}

diesel::table! {
    messages (id) {
        id -> Uuid,
        content -> Nullable<Text>,
        media -> Nullable<Array<Text>>,
        sent_at -> Timestamp,
        mentions -> Nullable<Array<Uuid>>,
        author -> Uuid,
        chat_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        pubkey -> Text,
        username -> Text,
        display_name -> Nullable<Text>,
        img_src -> Nullable<Text>,
        bio -> Nullable<Text>,
        contacts -> Array<Uuid>,
    }
}

diesel::joinable!(chat_member -> chats (chat_id));
diesel::joinable!(invite -> chats (chat_id));
diesel::joinable!(message_reaction -> messages (message_id));
diesel::joinable!(message_reaction -> users (user_id));
diesel::joinable!(messages -> chats (chat_id));
diesel::joinable!(messages -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    chat_member,
    chats,
    invite,
    message_reaction,
    messages,
    users,
);
