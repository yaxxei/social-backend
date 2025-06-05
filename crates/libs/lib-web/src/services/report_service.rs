use chrono::NaiveDateTime;
use lib_core::db::Db;
use lib_core::model::report::{
    ReportForCreate, ReportForDelete, ReportForSelect, ReportForUpdate, ReportRepo,
    ReportStatusType, ReportTargetType,
};
use serde::Serialize;
use uuid::Uuid;

use super::comment_service::{CommentDto, CommentService};
use super::post_service::{PostDto, PostService};
use super::user_service::{UserDto, UserService};

use crate::error::{Error, Result};

#[derive(Serialize, Clone)]
pub struct ReportDto {
    pub id: Uuid,
    pub report_type: ReportTargetType,
    pub reported_id: Uuid,
    pub reporter_id: Uuid,
    pub reason: Option<String>,
    pub status: ReportStatusType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub reporter: UserDto,
    pub reported_post: Option<PostDto>,
    pub reported_comment: Option<CommentDto>,
    pub reported_user: Option<UserDto>,
}

pub struct ReportService;

impl ReportService {
    pub async fn create(
        db: &Db,
        report_type: ReportTargetType,
        reported_id: &Uuid,
        reporter_id: &Uuid,
        reason: Option<String>,
    ) -> Result<ReportDto> {
        let report_fc = ReportForCreate {
            report_type,
            reported_id: *reported_id,
            reporter_id: *reporter_id,
            reason,
        };

        let report = ReportRepo::create(db, report_fc)
            .await
            .map_err(Error::Core)?;
        Self::convert_to_dto(db, report).await
    }

    pub async fn get_by_id(db: &Db, id: &Uuid) -> Result<ReportDto> {
        let report_fs = ReportForSelect {
            id: Some(*id),
            ..Default::default()
        };
        let report = ReportRepo::find(db, report_fs).await.map_err(Error::Core)?;
        Self::convert_to_dto(db, report).await
    }

    pub async fn get_many(db: &Db) -> Result<Vec<ReportDto>> {
        let report_fs = ReportForSelect {
            ..Default::default()
        };
        let reports = ReportRepo::find_many(db, report_fs)
            .await
            .map_err(Error::Core)?
            .into_iter()
            .map(|report| {
                let db = db.clone();
                async move { Self::convert_to_dto(&db, report).await }
            });

        futures::future::try_join_all(reports).await
    }

    pub async fn get_many_by_reported_id(db: &Db, reported_id: &Uuid) -> Result<Vec<ReportDto>> {
        let reports = ReportRepo::find_many(
            db,
            ReportForSelect {
                reported_id: Some(*reported_id),
                ..Default::default()
            },
        )
        .await
        .map_err(Error::Core)?
        .into_iter()
        .map(|report| {
            let db = db.clone();
            async move { Self::convert_to_dto(&db, report).await }
        });

        futures::future::try_join_all(reports).await
    }

    pub async fn get_many_by_reporter_id(db: &Db, reporter_id: &Uuid) -> Result<Vec<ReportDto>> {
        let report_fs = ReportForSelect {
            reporter_id: Some(*reporter_id),
            ..Default::default()
        };
        let reports = ReportRepo::find_many(db, report_fs)
            .await
            .map_err(Error::Core)?
            .into_iter()
            .map(|report| {
                let db = db.clone();
                async move { Self::convert_to_dto(&db, report).await }
            });

        futures::future::try_join_all(reports).await
    }

    pub async fn get_many_by_status(db: &Db, status: ReportStatusType) -> Result<Vec<ReportDto>> {
        let report_fs = ReportForSelect {
            status: Some(status),
            ..Default::default()
        };
        let reports = ReportRepo::find_many(db, report_fs)
            .await
            .map_err(Error::Core)?
            .into_iter()
            .map(|report| {
                let db = db.clone();
                async move { Self::convert_to_dto(&db, report).await }
            });

        futures::future::try_join_all(reports).await
    }

    pub async fn update_status(
        db: &Db,
        id: &Uuid,
        status: ReportStatusType,
        reason: Option<String>,
    ) -> Result<ReportDto> {
        let report_fu = ReportForUpdate {
            status: Some(status),
            reason,
        };

        let report = ReportRepo::update(db, id, report_fu)
            .await
            .map_err(Error::Core)?;
        Self::convert_to_dto(db, report).await
    }

    pub async fn delete(db: &Db, id: &Uuid) -> Result<()> {
        let report_fd = ReportForDelete { id: *id };
        ReportRepo::delete(db, report_fd).await.map_err(Error::Core)
    }

    async fn convert_to_dto(db: &Db, report: ReportRepo) -> Result<ReportDto> {
        let reporter = UserService::get_by_id(db, None, &report.reporter_id).await?;

        let (reported_post, reported_comment, reported_user) = match report.report_type {
            ReportTargetType::Post => (
                Some(PostService::get_by_id(db, None, &report.reported_id).await?),
                None,
                None,
            ),
            ReportTargetType::Comment => (
                None,
                Some(CommentService::get_by_id(db, None, &report.reported_id).await?),
                None,
            ),
            ReportTargetType::User => (
                None,
                None,
                Some(UserService::get_by_id(db, None, &report.reported_id).await?),
            ),
        };

        Ok(ReportDto {
            id: report.id,
            report_type: report.report_type,
            reported_id: report.reported_id,
            reporter_id: report.reporter_id,
            reason: report.reason,
            status: report.status,
            created_at: report.created_at,
            updated_at: report.updated_at,
            reporter,
            reported_post,
            reported_comment,
            reported_user,
        })
    }
}
