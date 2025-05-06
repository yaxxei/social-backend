-- Add up migration script here
CREATE TABLE IF NOT EXISTS comments (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    post_id UUID NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id UUID NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_comment_id UUID NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    CHECK (parent_comment_id IS NULL OR parent_comment_id <> id)
);

-- Trigger for automatic update updated_at
CREATE TRIGGER set_updated_at 
BEFORE UPDATE ON comments 
FOR EACH ROW EXECUTE 
FUNCTION update_updated_at_column();
