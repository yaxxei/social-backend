-- Add up migration script here
CREATE TABLE IF NOT EXISTS likes (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    post_id UUID NULL REFERENCES posts(id) ON DELETE CASCADE,
    comment_id UUID NULL REFERENCES comments(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    like_type SMALLINT NOT NULL CHECK (like_type IN (-1, 1)),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CHECK (
        (post_id IS NULL AND comment_id IS NOT NULL) OR
        (post_id IS NOT NULL AND comment_id IS NULL)
    )
);

-- Для лайков постов
CREATE UNIQUE INDEX likes_post_user_unique ON likes (post_id, user_id) 
WHERE post_id IS NOT NULL;

-- Для лайков комментариев
CREATE UNIQUE INDEX likes_comment_user_unique ON likes (comment_id, user_id) 
WHERE comment_id IS NOT NULL;

-- Trigger for automatic update updated_at
CREATE TRIGGER set_updated_at 
BEFORE UPDATE ON likes 
FOR EACH ROW EXECUTE 
FUNCTION update_updated_at_column();