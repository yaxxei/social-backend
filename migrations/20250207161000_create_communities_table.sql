-- Add up migration script here
CREATE TABLE IF NOT EXISTS communities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    is_private BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
