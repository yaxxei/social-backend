use crate::db::crud_fns::{create, delete, select, select_many, update};
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
}

#[derive(Serialize, Default)]
pub struct PostForDelete {
    pub id: Uuid,
}

impl PostRepo {
    pub async fn create(db: &Db, user_post_fc: PostForCreate) -> Result<PostRepo> {
        create::<Self, _>(db, user_post_fc).await
    }

    pub async fn update(db: &Db, id: &Uuid, user_post_fu: PostForUpdate) -> Result<PostRepo> {
        update::<Self, _>(db, id, user_post_fu).await
    }

    pub async fn find(db: &Db, user_post_fs: PostForSelect) -> Result<PostRepo> {
        select::<Self, _>(db, user_post_fs).await
    }

    pub async fn find_many(db: &Db, user_post_fs: PostForSelect) -> Result<Vec<PostRepo>> {
        select_many::<Self, _>(db, user_post_fs).await
    }

    pub async fn delete(db: &Db, user_post_fd: PostForDelete) -> Result<()> {
        delete::<Self, _>(db, user_post_fd).await
    }
}
