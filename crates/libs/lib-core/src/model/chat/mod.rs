use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::db::crud_fns::{create, delete, select, select_many, update};
use crate::db::{Db, DbEntity};
use crate::error::Result;

#[derive(Debug, FromRow, Serialize)]
pub struct ChatRepo {
    pub id: Uuid,
    pub name: Option<String>,
    pub is_group: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct ChatForCreate {
    pub name: Option<String>,
    pub is_group: bool,
}

#[derive(Serialize, Default)]
pub struct ChatForUpdate {
    pub name: Option<String>,
    pub is_group: Option<bool>,
}

#[derive(Serialize, Default)]
pub struct ChatForSelect {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub is_group: Option<bool>,
}

impl DbEntity for ChatRepo {
    const TABLE: &'static str = "chats";
}

impl ChatRepo {
    pub async fn create(db: &Db, data: ChatForCreate) -> Result<Self> {
        create::<Self, _>(db, data).await
    }

    pub async fn update(db: &Db, id: &Uuid, data: ChatForUpdate) -> Result<Self> {
        update::<Self, _>(db, id, data).await
    }

    pub async fn find(db: &Db, filter: ChatForSelect) -> Result<Self> {
        select::<Self, _>(db, filter).await
    }

    pub async fn find_all(db: &Db, filter: ChatForSelect) -> Result<Vec<Self>> {
        select_many::<Self, _>(db, filter).await
    }

    pub async fn find_many_by_query(db: &Db, query: &str) -> Result<Vec<Self>> {
        let q = format!("%{}%", query);

        let users = sqlx::query_as("SELECT * FROM chats WHERE name ILIKE $1")
            .bind(&q)
            .fetch_all(db)
            .await?;

        Ok(users)
    }

    pub async fn delete(db: &Db, id: &Uuid) -> Result<()> {
        delete::<Self, _>(db, id).await
    }
}
