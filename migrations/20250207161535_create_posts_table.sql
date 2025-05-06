-- Add up migration script here
CREATE TABLE IF NOT EXISTS posts (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Trigger for automatic update updated_at
CREATE TRIGGER set_updated_at 
BEFORE UPDATE ON posts 
FOR EACH ROW EXECUTE 
FUNCTION update_updated_at_column();
