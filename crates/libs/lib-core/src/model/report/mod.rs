use chrono::NaiveDateTime;
use derive_more::derive::Display;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::Type;
use std::str::FromStr;
use uuid::Uuid;

use crate::db::crud_fns::{create, delete, select, select_many, update};
use crate::db::{Db, DbEntity};
use crate::error::Error;
use crate::error::Result;

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy, Display)]
#[sqlx(type_name = "report_target_type")]
#[serde(rename_all = "snake_case")]
pub enum ReportTargetType {
    #[sqlx(rename = "post")]
    #[display("post")]
    Post,
    #[sqlx(rename = "comment")]
    #[display("comment")]
    Comment,
    #[sqlx(rename = "user")]
    #[display("user")]
    User,
}

impl FromStr for ReportTargetType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "post" => Ok(ReportTargetType::Post),
            "comment" => Ok(ReportTargetType::Comment),
            "user" => Ok(ReportTargetType::User),
            _ => Err(Error::ParseEnumError),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, Copy, Display)]
#[sqlx(type_name = "report_status_type")]
#[serde(rename_all = "snake_case")]
pub enum ReportStatusType {
    #[sqlx(rename = "pending")]
    #[display("pending")]
    Pending,
    #[sqlx(rename = "approved")]
    #[display("approved")]
    Approved,
    #[sqlx(rename = "rejected")]
    #[display("rejected")]
    Rejected,
    #[sqlx(rename = "processed")]
    #[display("processed")]
    Processed,
}

impl FromStr for ReportStatusType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "pending" => Ok(ReportStatusType::Pending),
            "approved" => Ok(ReportStatusType::Approved),
            "rejected" => Ok(ReportStatusType::Rejected),
            "processed" => Ok(ReportStatusType::Processed),
            _ => Err(Error::ParseEnumError),
        }
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ReportRepo {
    pub id: Uuid,
    pub report_type: ReportTargetType,
    pub reported_id: Uuid,
    pub reporter_id: Uuid,
    pub reason: Option<String>,
    pub status: ReportStatusType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl DbEntity for ReportRepo {
    const TABLE: &'static str = "reports";
}

#[derive(Serialize)]
pub struct ReportForCreate {
    pub report_type: ReportTargetType,
    pub reported_id: Uuid,
    pub reporter_id: Uuid,
    pub reason: Option<String>,
}

#[derive(Serialize, Default)]
pub struct ReportForUpdate {
    pub status: Option<ReportStatusType>,
    pub reason: Option<String>,
}

#[derive(Serialize, Default)]
pub struct ReportForSelect {
    pub id: Option<Uuid>,
    pub report_type: Option<ReportTargetType>,
    pub reported_id: Option<Uuid>,
    pub reporter_id: Option<Uuid>,
    pub status: Option<ReportStatusType>,
}

#[derive(Serialize, Default)]
pub struct ReportForDelete {
    pub id: Uuid,
}

impl ReportRepo {
    pub async fn create(db: &Db, report_fc: ReportForCreate) -> Result<ReportRepo> {
        create::<Self, _>(db, report_fc).await
    }

    pub async fn update(db: &Db, id: &Uuid, report_fu: ReportForUpdate) -> Result<ReportRepo> {
        update::<Self, _>(db, id, report_fu).await
    }

    pub async fn find(db: &Db, report_fs: ReportForSelect) -> Result<ReportRepo> {
        select::<Self, _>(db, report_fs).await
    }

    pub async fn find_many(db: &Db, report_fs: ReportForSelect) -> Result<Vec<ReportRepo>> {
        select_many::<Self, _>(db, report_fs).await
    }

    pub async fn delete(db: &Db, report_fd: ReportForDelete) -> Result<()> {
        delete::<Self, _>(db, report_fd).await
    }
}
