-- Add up migration script here
CREATE TYPE role_enum as ENUM ('admin', 'moderator', 'user');

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nickname TEXT NOT NULL UNIQUE,
    role role_enum NOT NULL DEFAULT 'user',
    email TEXT NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Trigger for automatic update updated_at
CREATE TRIGGER set_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();