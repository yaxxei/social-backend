use std::sync::Arc;

use axum::extract::{Path, State};
use lib_core::model::ModelManager;
use tracing::error;
use uuid::Uuid;

use crate::{
    extractors::CtxExt, services::follow_service::FollowService, utils::response::ApiResponse,
};

pub async fn follow(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(community_id): Path<Uuid>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Faield to follow";

    match FollowService::follow(&mm.db(), ctx.user_id, &community_id).await {
        Ok(_) => {
            // info!(
            //     "User {} successfully followed to entity {}",
            //     ctx.user_id, &community_id
            // );
            ()
        }
        Err(err) => {
            error!("Failed to follow: {}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    }

    ApiResponse::success(201, "Follow successful", None)
}

pub async fn unfollow(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(community_id): Path<Uuid>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Faield to unfollow";

    match FollowService::unfollow(&mm.db(), ctx.user_id, &community_id).await {
        Ok(_) => {
            // info!(
            //     "User {} successfully unfollowed from entity {}",
            //     ctx.user_id, community_id
            // );
            ()
        }
        Err(err) => {
            error!("Failed to unfollow: {}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    }

    ApiResponse::success(200, "Unfollow successful", None)
}

pub async fn is_followd(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(community_id): Path<Uuid>,
) -> ApiResponse<bool> {
    const FAILED_MESSAGE: &str = "Failed to check follow";

    match FollowService::is_followed(&mm.db(), ctx.user_id, &community_id).await {
        Ok(_) => {
            // info!(
            //     "Follow checked for user {} and community {}",
            //     ctx.user_id, community_id
            // );
            ()
        }
        Err(err) => {
            error!("Failed to check follow: {}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    }

    ApiResponse::success(200, "Unfollow successful", None)
}
