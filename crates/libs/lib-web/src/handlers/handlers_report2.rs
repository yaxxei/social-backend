use std::sync::Arc;

use axum::extract::{Path, State};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

use crate::{
    extractors::{CtxExt, ValidatedJson},
    services::report_service::{ReportDto, ReportService},
    utils::response::ApiResponse,
};
use super::AppState;

pub async fn get_reports(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
) -> ApiResponse<ReportsResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch reports";
    info!("Starting fetch reports");

    let reports = match ReportService::get_many(state.mm.db(), ctx.user_id).await {
        Ok(reports) => {
            info!("Successfully fetched {} reports", reports.len());
            reports
        }
        Err(err) => {
            error!("Failed to fetch reports: {:?}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let reports_response = ReportsResponse { reports };

    info!("Reports fetched successfully");
    ApiResponse::success(200, "Reports fetched successfully", Some(reports_response))
}

pub async fn get_report(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<ReportResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch report";
    info!("Starting fetch report by id: {}", id);

    let report = match ReportService::get_by_id(state.mm.db(), ctx.user_id, &id).await {
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

pub async fn create_report(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(payload): ValidatedJson<CreateReportPayload>,
) -> ApiResponse<ReportResponse> {
    const FAILED_MESSAGE: &str = "Failed to create report";
    info!("Starting create report by user: {:?}", ctx.user_id);

    let report = match ReportService::create(
        state.mm.db(),
        ctx.user_id,
        &payload.target_id,
        &payload.reason,
        payload.details.as_deref(),
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

pub async fn delete_report(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to delete report";
    info!("Starting delete report by user: {:?}", ctx.user_id);

    let _ = match ReportService::delete(state.mm.db(), ctx.user_id, &id).await {
        Ok(_) => {
            info!("Report deleted: {}", id);
        }
        Err(err) => {
            error!("Failed to delete report by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    info!("Report deleted successfully by user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Report deleted successfully", None)
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateReportPayload {
    target_id: Uuid,
    #[validate(length(min = 1))]
    reason: String,
    details: Option<String>,
}

#[derive(Serialize)]
pub struct ReportResponse {
    report: ReportDto,
}

#[derive(Serialize)]
pub struct ReportsResponse {
    reports: Vec<ReportDto>,
}