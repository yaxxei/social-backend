use crate::db::crud_fns::{count, create, delete, select, select_many};
use crate::db::{Db, DbEntity};
use crate::error::Result;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct FollowRepo {
    pub user_id: Uuid,
    pub community_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl DbEntity for FollowRepo {
    const TABLE: &'static str = "follows";
}

#[derive(Serialize)]
pub struct FollowForCreate {
    pub user_id: Uuid,
    pub community_id: Uuid,
}

#[derive(Serialize, Default)]
pub struct FollowForSelect {
    pub user_id: Option<Uuid>,
    pub community_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct FollowForDelete {
    pub user_id: Uuid,
    pub community_id: Uuid,
}

impl FollowRepo {
    pub async fn create(db: &Db, follow_fc: FollowForCreate) -> Result<FollowRepo> {
        create::<Self, _>(db, follow_fc).await
    }

    pub async fn find(db: &Db, follow_fs: FollowForSelect) -> Result<FollowRepo> {
        select::<Self, _>(db, follow_fs).await
    }

    pub async fn find_many(db: &Db, follow_fs: FollowForSelect) -> Result<Vec<FollowRepo>> {
        select_many::<Self, _>(db, follow_fs).await
    }

    pub async fn count(db: &Db, follow_fs: FollowForSelect) -> Result<usize> {
        count::<Self, _>(db, follow_fs).await
    }

    pub async fn delete(db: &Db, follow_fd: FollowForDelete) -> Result<()> {
        delete::<Self, _>(db, follow_fd).await
    }
}
