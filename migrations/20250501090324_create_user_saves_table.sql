-- Add migration script here
CREATE TABLE IF NOT EXISTS user_saves (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    user_id UUID NULL REFERENCES users(id) ON DELETE CASCADE,
    post_id UUID NULL REFERENCES posts(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);