use std::sync::Arc;

use axum::extract::State;
use lib_core::model::ModelManager;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

use crate::{
    extractors::{CtxExt, ValidatedJson},
    services::profile_service::{ProfileService, SaveDto},
    utils::response::ApiResponse,
};

pub async fn get_saves(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
) -> ApiResponse<SavesResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch saves";
    info!("Starting fetch saves");

    let saves = match ProfileService::get_saves(mm.db(), ctx.user_id).await {
        Ok(saves) => {
            info!("Successfully fetched {} saves", saves.len(),);
            saves
        }
        Err(err) => {
            error!("Failed to fetch saves: {:?}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let saves_response = SavesResponse { saves };

    info!("Saves fetched successfully");
    ApiResponse::success(200, "Saves fetched successully", Some(saves_response))
}

pub async fn create_save(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(paylaod): ValidatedJson<CreateSavePayload>,
) -> ApiResponse<SaveResponse> {
    const FAILED_MESSAGE: &str = "Failed to create save";
    info!("Starting craete save");

    let save = match ProfileService::create_save(mm.db(), ctx.user_id, &paylaod.post_id).await {
        Ok(save) => {
            info!("Successfully created save");
            save
        }
        Err(err) => {
            error!("Failed to create save: {:?}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let save_response = SaveResponse { save };

    info!("Save created successfully");
    ApiResponse::success(200, "Save created successully", Some(save_response))
}

pub async fn delete_save(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(paylaod): ValidatedJson<CreateSavePayload>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to delete save";
    info!("Starting delete save");

    let _ = match ProfileService::delete_save(mm.db(), ctx.user_id, &paylaod.post_id).await {
        Ok(_) => {
            info!("Successfully deleted save");
        }
        Err(err) => {
            error!("Failed to delete save: {:?}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    info!("Save deleted successfully");
    ApiResponse::success(200, "Save deleted successully", None)
}

#[derive(Deserialize, Validate)]
pub struct CreateSavePayload {
    post_id: Uuid,
}

#[derive(Serialize)]
pub struct SaveResponse {
    save: SaveDto,
}

#[derive(Serialize)]
pub struct SavesResponse {
    saves: Vec<SaveDto>,
}
