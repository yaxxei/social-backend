use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

use crate::db::crud_fns::{create, delete, select, select_many, update};
use crate::db::{Db, DbEntity};
use crate::error::Result;
use crate::model::role::RoleEnum;

#[derive(Debug, FromRow, Serialize)]
pub struct UserRepo {
    pub id: Uuid,
    pub nickname: String,
    pub role: RoleEnum,
    pub email: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct UserForCreate {
    pub nickname: String,
    pub email: String,
    pub hashed_password: String,
}

#[derive(Serialize, Default)]
pub struct UserForUpdate {
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub role: Option<RoleEnum>,
    pub hashed_password: Option<String>,
}

#[derive(Serialize, Default)]
pub struct UserForSelect {
    pub id: Option<Uuid>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub role: Option<RoleEnum>,
}

impl DbEntity for UserRepo {
    const TABLE: &'static str = "users";
}

impl UserRepo {
    pub async fn create(db: &Db, user_fc: UserForCreate) -> Result<UserRepo> {
        create::<Self, _>(db, user_fc).await
    }

    pub async fn update(db: &Db, id: &Uuid, user_fu: UserForUpdate) -> Result<UserRepo> {
        update::<Self, _>(db, id, user_fu).await
    }

    pub async fn find(db: &Db, user_fs: UserForSelect) -> Result<UserRepo> {
        select::<Self, _>(db, user_fs).await
    }

    pub async fn find_all(db: &Db, user_fs: UserForSelect) -> Result<Vec<UserRepo>> {
        select_many::<Self, _>(db, user_fs).await
    }

    pub async fn delete(db: &Db, id: &Uuid) -> Result<()> {
        delete::<Self, _>(db, id).await
    }
}
