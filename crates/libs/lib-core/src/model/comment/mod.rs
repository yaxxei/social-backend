use crate::db::crud_fns::{count, select, select_many, update};
use crate::db::{crud_fns::create, Db, DbEntity};
use crate::error::Result;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct CommentRepo {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub parent_comment_id: Option<Uuid>,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_deleted: bool,
}

impl DbEntity for CommentRepo {
    const TABLE: &'static str = "comments";
}

#[derive(Serialize)]
pub struct CommentForCreate {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
}

#[derive(Serialize, Default)]
pub struct CommentForSelect {
    pub id: Option<Uuid>,
    pub post_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub parent_comment_id: Option<Uuid>,
}

#[derive(Serialize, Default)]
pub struct CommentForUpdate {
    pub content: Option<String>,
}

#[derive(Serialize)]
pub struct CommentForDelete {
    pub id: Uuid,
}

impl CommentRepo {
    pub async fn create(db: &Db, comment_fc: CommentForCreate) -> Result<CommentRepo> {
        create::<Self, _>(db, comment_fc).await
    }

    pub async fn find(db: &Db, comment_fc: CommentForSelect) -> Result<CommentRepo> {
        select::<Self, _>(db, comment_fc).await
    }

    pub async fn find_many(db: &Db, comment_fs: CommentForSelect) -> Result<Vec<CommentRepo>> {
        select_many::<Self, _>(db, comment_fs).await
    }

    pub async fn update(db: &Db, id: &Uuid, comment_fu: CommentForUpdate) -> Result<CommentRepo> {
        update::<Self, _>(db, id, comment_fu).await
    }

    pub async fn delete(db: &Db, comment_fd: CommentForDelete) -> Result<()> {
        // delete::<Self, _>(db, comment_fd).await
        let query = "UPDATE comments SET is_deleted = TRUE, content = '' WHERE id = $1";
        let _ = sqlx::query(query).bind(comment_fd.id).execute(db).await?;

        Ok(())
    }

    pub async fn count(db: &Db, comment_fs: CommentForSelect) -> Result<usize> {
        count::<Self, _>(db, comment_fs).await
    }
}
