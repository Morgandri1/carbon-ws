-- Drop tables in reverse order to handle dependencies
DROP TABLE IF EXISTS invite;
DROP TABLE IF EXISTS chat_member;
DROP TABLE IF EXISTS message_reaction;
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS chats;
DROP TABLE IF EXISTS users;