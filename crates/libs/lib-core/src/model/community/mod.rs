use crate::db::crud_fns::{create, delete, select, select_many, update};
use crate::db::{Db, DbEntity};
use crate::error::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CommunityRepo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub is_private: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct CommunityForCreate {
    pub name: String,
    pub user_id: Uuid,
    pub description: String,
    pub is_private: bool,
}

#[derive(Serialize, Default)]
pub struct CommunityForUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_private: Option<bool>,
}

#[derive(Serialize, Default)]
pub struct CommunityForSelect {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_private: Option<bool>,
}

#[derive(Serialize, Default)]
pub struct CommunityForDelete {
    pub id: Uuid,
}

impl DbEntity for CommunityRepo {
    const TABLE: &'static str = "communities";
}

impl CommunityRepo {
    pub async fn create(db: &Db, community_fc: CommunityForCreate) -> Result<CommunityRepo> {
        create::<Self, _>(db, community_fc).await
    }

    pub async fn update(
        db: &Db,
        id: &Uuid,
        community_fu: CommunityForUpdate,
    ) -> Result<CommunityRepo> {
        update::<Self, _>(db, id, community_fu).await
    }

    pub async fn find(db: &Db, community_fs: CommunityForSelect) -> Result<CommunityRepo> {
        select::<Self, _>(db, community_fs).await
    }

    pub async fn find_all(db: &Db, user_fs: CommunityForSelect) -> Result<Vec<CommunityRepo>> {
        select_many::<Self, _>(db, user_fs).await
    }

    pub async fn find_many_by_query(db: &Db, query: &str) -> Result<Vec<CommunityRepo>> {
        let q = format!("%{}%", query);

        let users = sqlx::query_as("SELECT * FROM communities WHERE name ILIKE $1")
            .bind(&q)
            .fetch_all(db)
            .await?;

        Ok(users)
    }

    pub async fn delete(db: &Db, community_fd: CommunityForDelete) -> Result<()> {
        delete::<Self, _>(db, community_fd).await
    }
}
