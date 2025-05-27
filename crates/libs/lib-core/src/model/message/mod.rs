use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::db::crud_fns::{create, select, select_many, update};
use crate::db::{Db, DbEntity};
use crate::error::Result;

#[derive(Debug, FromRow, Serialize)]
pub struct MessageRepo {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_deleted: bool,
}

#[derive(Serialize)]
pub struct MessageForCreate {
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
}

#[derive(Serialize, Default)]
pub struct MessageForUpdate {
    pub content: Option<String>,
    pub is_deleted: Option<bool>,
}

#[derive(Serialize, Default)]
pub struct MessageForSelect {
    pub id: Option<Uuid>,
    pub chat_id: Option<Uuid>,
    pub sender_id: Option<Uuid>,
}

impl DbEntity for MessageRepo {
    const TABLE: &'static str = "messages";
}

impl MessageRepo {
    pub async fn create(db: &Db, data: MessageForCreate) -> Result<Self> {
        create::<Self, _>(db, data).await
    }

    pub async fn update(db: &Db, id: &Uuid, data: MessageForUpdate) -> Result<Self> {
        update::<Self, _>(db, id, data).await
    }

    pub async fn find(db: &Db, filter: MessageForSelect) -> Result<Self> {
        select::<Self, _>(db, filter).await
    }

    pub async fn find_last_by_chat(db: &Db, chat_id: &Uuid) -> Result<Option<Self>> {
        let query = r#"
            SELECT * 
            FROM messages
            WHERE chat_id = $1
            ORDER BY created_at DESC
            LIMIT 1;
        "#;

        let message = sqlx::query_as(query)
            .bind(chat_id)
            .fetch_optional(db)
            .await?;
        Ok(message)
    }

    pub async fn find_all(db: &Db, filter: MessageForSelect) -> Result<Vec<Self>> {
        select_many::<Self, _>(db, filter).await
    }

    pub async fn find_many_by_query(db: &Db, query: &str) -> Result<Vec<Self>> {
        let q = format!("%{}%", query);

        let users = sqlx::query_as("SELECT * FROM messages WHERE content ILIKE $1")
            .bind(&q)
            .fetch_all(db)
            .await?;

        Ok(users)
    }

    pub async fn delete(db: &Db, id: &Uuid) -> Result<Self> {
        // delete::<Self, _>(db, id).await
        let query = "UPDATE messages SET is_deleted = TRUE, content = '' WHERE id = $1";
        let message = sqlx::query_as(query).bind(id).fetch_one(db).await?;
        Ok(message)
    }
}
