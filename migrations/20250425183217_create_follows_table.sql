-- Add migration script here
CREATE TABLE IF NOT EXISTS follows (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    user_id UUID NULL REFERENCES users(id) ON DELETE CASCADE,
    community_id UUID NULL REFERENCES communities(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Trigger for automatic update updated_at
CREATE TRIGGER set_updated_at 
BEFORE UPDATE ON follows 
FOR EACH ROW EXECUTE 
FUNCTION update_updated_at_column();