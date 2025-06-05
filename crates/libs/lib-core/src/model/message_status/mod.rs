use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::db::crud_fns::{create, delete, select, select_many, update};
use crate::db::{Db, DbEntity};
use crate::error::Result;

#[derive(Debug, FromRow, Serialize)]
pub struct MessageStatusRepo {
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub chat_id: Uuid,
    pub is_send: bool,
    pub is_read: bool,
    pub read_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct MessageStatusForCreate {
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub chat_id: Uuid,
    pub is_send: bool,
    pub is_read: bool,
    pub read_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Default)]
pub struct MessageStatusForUpdate {
    pub is_send: Option<bool>,
    pub is_read: Option<bool>,
    pub read_at: Option<Option<NaiveDateTime>>,
}

#[derive(Serialize, Default)]
pub struct MessageStatusForSelect {
    pub message_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub chat_id: Option<Uuid>,
    pub is_read: Option<bool>,
}

impl DbEntity for MessageStatusRepo {
    const TABLE: &'static str = "message_statuses";
}

impl MessageStatusRepo {
    pub async fn create(db: &Db, data: MessageStatusForCreate) -> Result<Self> {
        create::<Self, _>(db, data).await
    }

    pub async fn update(db: &Db, id: &Uuid, data: MessageStatusForUpdate) -> Result<Self> {
        update::<Self, _>(db, id, data).await
    }

    pub async fn read_message(db: &Db, msg_id: &Uuid, user_id: &Uuid) -> Result<()> {
        let query =
            "UPDATE message_statuses SET is_read = TRUE WHERE message_id = $1 AND user_id = $2";
        let _ = sqlx::query(query)
            .bind(msg_id)
            .bind(user_id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn find(db: &Db, filter: MessageStatusForSelect) -> Result<Self> {
        select::<Self, _>(db, filter).await
    }

    pub async fn find_all(db: &Db, filter: MessageStatusForSelect) -> Result<Vec<Self>> {
        select_many::<Self, _>(db, filter).await
    }

    pub async fn delete(db: &Db, id: &Uuid) -> Result<()> {
        delete::<Self, _>(db, id).await
    }
}
