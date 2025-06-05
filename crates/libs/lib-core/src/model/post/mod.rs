use crate::db::crud_fns::{create, select, select_many, update};
use crate::db::{Db, DbEntity};
use crate::error::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct PostRepo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub community_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_deleted: bool,
}

impl DbEntity for PostRepo {
    const TABLE: &'static str = "posts";
}

#[derive(Serialize)]
pub struct PostForCreate {
    pub user_id: Uuid,
    pub community_id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Default)]
pub struct PostForUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Default)]
pub struct PostForSelect {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub community_id: Option<Uuid>,
    pub is_deleted: Option<bool>,
}

#[derive(Serialize, Default)]
pub struct PostForDelete {
    pub id: Uuid,
}

impl PostRepo {
    pub async fn create(db: &Db, post_fc: PostForCreate) -> Result<PostRepo> {
        create::<Self, _>(db, post_fc).await
    }

    pub async fn update(db: &Db, id: &Uuid, post_fu: PostForUpdate) -> Result<PostRepo> {
        update::<Self, _>(db, id, post_fu).await
    }

    pub async fn find(db: &Db, post_fs: PostForSelect) -> Result<PostRepo> {
        select::<Self, _>(db, post_fs).await
    }

    pub async fn find_many(db: &Db, post_fs: PostForSelect) -> Result<Vec<PostRepo>> {
        select_many::<Self, _>(db, post_fs).await
    }

    pub async fn find_many_by_query(db: &Db, query: &str) -> Result<Vec<PostRepo>> {
        let q = format!("%{}%", query);

        let users = sqlx::query_as("SELECT * FROM posts WHERE title ILIKE $1")
            .bind(&q)
            .fetch_all(db)
            .await?;

        Ok(users)
    }

    pub async fn delete(db: &Db, post_fd: PostForDelete) -> Result<Self> {
        let query = "UPDATE posts SET is_deleted = TRUE, content = '' WHERE id = $1";
        let post = sqlx::query_as(query).bind(post_fd.id).fetch_one(db).await?;
        Ok(post)
    }
}
