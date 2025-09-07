-- Create extension for UUID support
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pubkey TEXT NOT NULL,
    username TEXT NOT NULL,
    display_name TEXT,
    img_src TEXT,
    bio TEXT,
    contacts UUID[] NOT NULL DEFAULT '{}'
);

-- Create chats table
CREATE TABLE chats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    broadcast BOOLEAN NOT NULL,
    img_src TEXT,
    description TEXT,
    admins UUID[]
);

-- Create messages table
CREATE TABLE messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content TEXT,
    media TEXT[] NOT NULL DEFAULT '{}',
    sent_at TIMESTAMP NOT NULL,
    mentions UUID[] NOT NULL DEFAULT '{}',
    author UUID NOT NULL,
    chat_id UUID NOT NULL,
    FOREIGN KEY (author) REFERENCES users (id),
    FOREIGN KEY (chat_id) REFERENCES chats (id)
);

-- Create message_reaction table
CREATE TABLE message_reaction (
    message_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    reaction TEXT NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

-- Create chat_member table
CREATE TABLE chat_member (
    chat_id UUID NOT NULL,
    user_id UUID NOT NULL,
    joined_at TIMESTAMP NOT NULL,
    invited_by UUID,
    symkey TEXT,
    PRIMARY KEY (chat_id, user_id),
    FOREIGN KEY (chat_id) REFERENCES chats (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (invited_by) REFERENCES users (id)
);

-- Create invite table
CREATE TABLE invite (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    from_user UUID NOT NULL,
    to_user UUID NOT NULL,
    symkey TEXT NOT NULL,
    chat_id UUID NOT NULL,
    FOREIGN KEY (from_user) REFERENCES users (id),
    FOREIGN KEY (to_user) REFERENCES users (id),
    FOREIGN KEY (chat_id) REFERENCES chats (id)
);

-- Create indexes for foreign keys for better performance
CREATE INDEX messages_author_idx ON messages (author);
CREATE INDEX messages_chat_id_idx ON messages (chat_id);
CREATE INDEX message_reaction_user_id_idx ON message_reaction (user_id);
CREATE INDEX chat_member_user_id_idx ON chat_member (user_id);
CREATE INDEX chat_member_invited_by_idx ON chat_member (invited_by);
CREATE INDEX invite_from_user_idx ON invite (from_user);
CREATE INDEX invite_to_user_idx ON invite (to_user);
CREATE INDEX invite_chat_id_idx ON invite (chat_id);