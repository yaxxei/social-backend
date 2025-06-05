use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::db::crud_fns::{create, delete, select, select_many, update};
use crate::db::{Db, DbEntity};
use crate::error::Result;

use super::chat_role::ChatRoleEnum;

#[derive(Debug, FromRow, Serialize, PartialEq)]
pub struct ChatMemberRepo {
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: NaiveDateTime,
    pub role: ChatRoleEnum,
}

#[derive(Serialize)]
pub struct ChatMemberForCreate {
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub role: Option<ChatRoleEnum>,
}

#[derive(Serialize, Default)]
pub struct ChatMemberForUpdate {
    pub role: Option<ChatRoleEnum>,
}

#[derive(Serialize, Default)]
pub struct ChatMemberForSelect {
    pub chat_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub role: Option<ChatRoleEnum>,
}

#[derive(Serialize, Default)]
pub struct ChatMemberForDelete {
    pub chat_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

impl DbEntity for ChatMemberRepo {
    const TABLE: &'static str = "chat_members";
}

impl ChatMemberRepo {
    pub async fn create(db: &Db, data: ChatMemberForCreate) -> Result<Self> {
        create::<Self, _>(db, data).await
    }

    pub async fn update(db: &Db, id: &Uuid, data: ChatMemberForUpdate) -> Result<Self> {
        update::<Self, _>(db, id, data).await
    }

    pub async fn find(db: &Db, filter: ChatMemberForSelect) -> Result<Self> {
        select::<Self, _>(db, filter).await
    }

    pub async fn find_all(db: &Db, filter: ChatMemberForSelect) -> Result<Vec<Self>> {
        select_many::<Self, _>(db, filter).await
    }

    pub async fn delete(db: &Db, filter: ChatMemberForDelete) -> Result<()> {
        delete::<Self, _>(db, filter).await
    }
}
