use crate::db::crud_fns::{create, delete, select, select_many};
use crate::db::{Db, DbEntity};
use crate::error::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SaveRepo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub created_at: NaiveDateTime,
}

impl DbEntity for SaveRepo {
    const TABLE: &'static str = "user_saves";
}

#[derive(Serialize)]
pub struct SaveForCreate {
    pub user_id: Uuid,
    pub post_id: Uuid,
}

#[derive(Serialize, Default)]
pub struct SaveForSelect {
    pub user_id: Option<Uuid>,
    pub post_id: Option<Uuid>,
}

#[derive(Serialize, Default)]
pub struct SaveForDelete {
    pub user_id: Uuid,
    pub post_id: Uuid,
}

impl SaveRepo {
    pub async fn create(db: &Db, save_fc: SaveForCreate) -> Result<SaveRepo> {
        create::<Self, _>(db, save_fc).await
    }

    pub async fn find(db: &Db, save_fs: SaveForSelect) -> Result<SaveRepo> {
        select::<Self, _>(db, save_fs).await
    }

    pub async fn find_many(db: &Db, save_fs: SaveForSelect) -> Result<Vec<SaveRepo>> {
        select_many::<Self, _>(db, save_fs).await
    }

    pub async fn delete(db: &Db, save_fd: SaveForDelete) -> Result<()> {
        delete::<Self, _>(db, save_fd).await
    }
}
