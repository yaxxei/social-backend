use std::sync::Arc;

use crate::{
    extractors::{CtxExt, ValidatedJson},
    services::community_service::{CommunityDto, CommunityService},
    utils::response::ApiResponse,
};
use axum::extract::{Path, Query, State};
use lib_core::model::ModelManager;
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};
use uuid::Uuid;
use validator::Validate;

#[instrument(skip(mm))]
pub async fn get_communities(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<CommunityParam>,
) -> ApiResponse<CommunitiesResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch communities";

    let communities = match params.user_id {
        Some(user_id) => {
            match CommunityService::get_many_by_user_id(mm.db(), ctx.user_id, &user_id).await {
                Ok(community) => {
                    info!("Communities fetched");
                    community
                }
                Err(err) => {
                    error!("Failed to fetch communities");
                    return ApiResponse::error(FAILED_MESSAGE, err);
                }
            }
        }
        None => match CommunityService::get_many(mm.db(), ctx.user_id).await {
            Ok(community) => {
                info!("Communities fetched");
                community
            }
            Err(err) => {
                error!("Failed to fetch communities");
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        },
    };

    let community_response = CommunitiesResponse { communities };

    info!("Communities fetched successfully");
    ApiResponse::success(
        200,
        "Communities fetched successfully",
        Some(community_response),
    )
}

#[instrument(skip(mm))]
pub async fn get(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(name): Path<String>,
) -> ApiResponse<CommunityResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch community";

    let community = match CommunityService::get_by_name(mm.db(), ctx.user_id, &name).await {
        Ok(community) => {
            info!("Community found: {}", community.name);
            community
        }
        Err(err) => {
            error!("Failed to found community: {}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let community_response = CommunityResponse { community };

    info!(
        "Community successfully fetched by id: {}",
        community_response.community.id
    );
    ApiResponse::success(
        200,
        "Community fetched successfully",
        Some(community_response),
    )
}

#[instrument(skip(mm))]
pub async fn create(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(payload): ValidatedJson<CreatePayload>,
) -> ApiResponse<CommunityResponse> {
    const FAILED_MESSAGE: &str = "Failed to create community";
    info!("Starting create community method");

    let community = match CommunityService::create(
        mm.db(),
        ctx.user_id,
        &payload.name,
        &payload.description,
        &payload.is_private,
    )
    .await
    {
        Ok(community) => {
            info!("Community created: {}", community.name);
            community
        }
        Err(err) => {
            error!("Failed to create community for user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let community_response = CommunityResponse { community };

    info!("Community successfully created for user: {:?}", ctx.user_id);
    ApiResponse::success(
        201,
        "Community created successfully",
        Some(community_response),
    )
}

#[instrument(skip(mm))]
pub async fn update(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(name): Path<String>,
    ValidatedJson(payload): ValidatedJson<UpdateCommunityPayload>,
) -> ApiResponse<CommunityResponse> {
    const FAILED_MESSAGE: &str = "Failed to udpate community";

    let community = match CommunityService::update_by_name(
        mm.db(),
        ctx.user_id,
        &name,
        payload.name,
        payload.description,
        payload.is_private,
    )
    .await
    {
        Ok(community) => {
            info!("Community {} updated", community.id);
            community
        }
        Err(err) => {
            error!("Failed to update community: {}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let community_response = CommunityResponse { community };

    info!(
        "Community updated successully: {}",
        community_response.community.name
    );
    ApiResponse::success(
        200,
        "Community name changed successfully",
        Some(community_response),
    )
}

#[instrument(skip(mm))]
pub async fn delete(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(name): Path<String>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to delete community";

    match CommunityService::delete_by_name(mm.db(), ctx.user_id, &name).await {
        Ok(_) => {
            info!("Community deleted successfully");
            return ApiResponse::success(200, "Community deleted successfully", None);
        }
        Err(err) => {
            error!("Failed to delete community");
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CommunityParam {
    user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePayload {
    name: String,
    description: String,
    is_private: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCommunityPayload {
    name: Option<String>,
    description: Option<String>,
    is_private: Option<bool>,
}

#[derive(Serialize)]
pub struct CommunityResponse {
    community: CommunityDto,
}

#[derive(Serialize)]
pub struct CommunitiesResponse {
    communities: Vec<CommunityDto>,
}
