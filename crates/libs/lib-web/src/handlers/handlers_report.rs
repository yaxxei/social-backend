use std::sync::Arc;

use axum::extract::{Path, Query, State};
use lib_core::model::report::{ReportStatusType, ReportTargetType};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::Error,
    extractors::{CtxExt, ValidatedJson},
    services::report_service::{ReportDto, ReportService},
    utils::response::ApiResponse,
};

use super::AppState;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum OutgoingWsMessage {
    #[serde(rename = "new_report")]
    NewReport { report: ReportDto },
}

pub async fn create_report(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(payload): ValidatedJson<CreateReportPayload>,
) -> ApiResponse<ReportResponse> {
    const FAILED_MESSAGE: &str = "Failed to create report";
    info!("Starting create report by user: {:?}", ctx.user_id);

    let requester_id = match ctx.user_id.ok_or(Error::Unauthorized) {
        Ok(uid) => uid,
        Err(e) => {
            error!("Requester Unauthorized");
            return ApiResponse::error(FAILED_MESSAGE, e);
        }
    };

    let report = match ReportService::create(
        state.mm.db(),
        payload.report_type,
        &payload.reported_id,
        &requester_id,
        payload.reason.clone(),
    )
    .await
    {
        Ok(report) => {
            info!("Report created: {}", report.id);
            report
        }
        Err(err) => {
            error!("Failed to create report by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let report_response = ReportResponse { report };

    info!("Report created successfully by user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Report created successfully", Some(report_response))
}

pub async fn get_report(
    State(state): State<Arc<AppState>>,
    CtxExt(_): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<ReportResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch report";
    info!("Starting fetch report by id: {}", id);

    let report = match ReportService::get_by_id(state.mm.db(), &id).await {
        Ok(report) => {
            info!("Report found: {}", id);
            report
        }
        Err(err) => {
            error!("Failed to find report by id: {}", id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let report_response = ReportResponse { report };

    info!("Report fetched successfully by id: {}", id);
    ApiResponse::success(200, "Report fetched successfully", Some(report_response))
}

pub async fn get_reports(
    State(state): State<Arc<AppState>>,
    CtxExt(_): CtxExt,
    Query(params): Query<ReportQuery>,
) -> ApiResponse<ReportsResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch reports";
    info!("Starting fetch reports");

    let reports = if let Some(reported_id) = params.reported_id {
        match ReportService::get_many_by_reported_id(state.mm.db(), &reported_id).await {
            Ok(reports) => {
                info!(
                    "Successfully fetched {} reports for reported_id: {}",
                    reports.len(),
                    reported_id
                );
                reports
            }
            Err(err) => {
                error!(
                    "Failed to fetch reports for reported_id {}: {:?}",
                    reported_id, err
                );
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        }
    } else if let Some(reporter_id) = params.reporter_id {
        match ReportService::get_many_by_reporter_id(state.mm.db(), &reporter_id).await {
            Ok(reports) => {
                info!(
                    "Successfully fetched {} reports for reporter_id: {}",
                    reports.len(),
                    reporter_id
                );
                reports
            }
            Err(err) => {
                error!(
                    "Failed to fetch reports for reporter_id {}: {:?}",
                    reporter_id, err
                );
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        }
    } else if let Some(status) = params.status {
        match ReportService::get_many_by_status(state.mm.db(), status).await {
            Ok(reports) => {
                info!(
                    "Successfully fetched {} reports with status: {}",
                    reports.len(),
                    status
                );
                reports
            }
            Err(err) => {
                error!("Failed to fetch reports with status {}: {:?}", status, err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        }
    } else {
        match ReportService::get_many(state.mm.db()).await {
            Ok(reports) => {
                info!("Successfully fetched {} reports", reports.len());
                reports
            }
            Err(err) => {
                error!("Failed to fetch reports: {:?}", err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        }
    };

    let reports_response = ReportsResponse { reports };

    info!("Reports fetched successfully");
    ApiResponse::success(200, "Reports fetched successfully", Some(reports_response))
}

pub async fn update_report_status(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateReportStatusPayload>,
) -> ApiResponse<ReportResponse> {
    const FAILED_MESSAGE: &str = "Failed to update report status";
    info!("Starting update report status by user: {:?}", ctx.user_id);

    let report = match ReportService::update_status(
        state.mm.db(),
        &id,
        payload.status,
        payload.reason.clone(),
    )
    .await
    {
        Ok(report) => {
            info!("Report status updated: {}", report.id);
            report
        }
        Err(err) => {
            error!("Failed to update report status by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let report_response = ReportResponse { report };

    info!(
        "Report status updated successfully by user: {:?}",
        ctx.user_id
    );
    ApiResponse::success(
        200,
        "Report status updated successfully",
        Some(report_response),
    )
}

pub async fn delete_report(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to delete report";
    info!("Starting delete report by user: {:?}", ctx.user_id);

    match ReportService::delete(state.mm.db(), &id).await {
        Ok(_) => {
            info!("Report deleted: {}", id);
            ApiResponse::success(200, "Report deleted successfully", None)
        }
        Err(err) => {
            error!("Failed to delete report by user: {:?}", ctx.user_id);
            ApiResponse::error(FAILED_MESSAGE, err)
        }
    }
}

#[derive(Deserialize)]
pub struct ReportQuery {
    reported_id: Option<Uuid>,
    reporter_id: Option<Uuid>,
    status: Option<ReportStatusType>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReportPayload {
    report_type: ReportTargetType,
    reported_id: Uuid,
    reason: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateReportStatusPayload {
    status: ReportStatusType,
    reason: Option<String>,
}

#[derive(Serialize)]
pub struct ReportResponse {
    report: ReportDto,
}

#[derive(Serialize)]
pub struct ReportsResponse {
    reports: Vec<ReportDto>,
}
