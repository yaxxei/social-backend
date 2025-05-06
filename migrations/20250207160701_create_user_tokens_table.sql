-- Add up migration script here
CREATE TYPE token_type_enum as ENUM ('access', 'refresh', 'reset_password', 'reset_email', 'email_verification');

CREATE TABLE IF NOT EXISTS user_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_type token_type_enum NOT NULL,
    token TEXT NOT NULL,
    UNIQUE (user_id, token_type) -- todo: need to rerun this migration
);