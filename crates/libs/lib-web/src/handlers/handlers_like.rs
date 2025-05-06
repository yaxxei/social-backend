use axum::extract::{Query, State};
use lib_core::model::{like::LikeRepo, ModelManager};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::Error, extractors::CtxExt, services::like_service::LikeService,
    utils::response::ApiResponse,
};
use tracing::{error, info};

pub async fn like(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<LikeQuery>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to like";
    info!("Starting like");

    if let Some(post_id) = params.post_id {
        let _ = match LikeService::like_post(&mm.db(), ctx.user_id, &post_id, 1).await {
            Ok(_) => {
                info!("Post liked successfully");
                return ApiResponse::success(201, "Post liked successfully", None);
            }
            Err(err) => {
                error!("Failed to like post: {:?}", err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };
    } else if let Some(comment_id) = params.comment_id {
        let _ = match LikeService::like_comment(&mm.db(), ctx.user_id, &comment_id, 1).await {
            Ok(_) => {
                info!("Comment liked successfully");
                return ApiResponse::success(201, "Comment liked successfully", None);
            }
            Err(err) => {
                error!("Failed to like comment: {:?}", err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };
    } else {
        error!("Failed to like: Missing post_id or comment_id query params");
        return ApiResponse::error(FAILED_MESSAGE, Error::MissingQuery);
    }
}

pub async fn dislike(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<LikeQuery>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to dislike";
    info!("Starting dislike");

    if let Some(post_id) = params.post_id {
        let _ = match LikeService::like_post(&mm.db(), ctx.user_id, &post_id, -1).await {
            Ok(_) => {
                info!("Post disliked successfully");
                return ApiResponse::success(201, "Post disliked successfully", None);
            }
            Err(err) => {
                error!("Failed to dislike post: {:?}", err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };
    } else if let Some(comment_id) = params.comment_id {
        let _ = match LikeService::like_comment(&mm.db(), ctx.user_id, &comment_id, -1).await {
            Ok(_) => {
                info!("Comment disliked successfully");
                return ApiResponse::success(201, "Comment disliked successfully", None);
            }
            Err(err) => {
                error!("Failed to dislike comment: {:?}", err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };
    } else {
        error!("Failed to dislike: Missing post_id or comment_id query params");
        return ApiResponse::error(FAILED_MESSAGE, Error::MissingQuery);
    }
}

pub async fn unlike(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<LikeQuery>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to unlike";
    info!("Starting unlike");

    if let Some(post_id) = params.post_id {
        let _ = match LikeService::unlike_post(&mm.db(), ctx.user_id, &post_id).await {
            Ok(_) => {
                info!("Post unliked successfully");
                return ApiResponse::success(201, "Post unliked successfully", None);
            }
            Err(err) => {
                error!("Failed to unlike post: {:?}", err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };
    } else if let Some(comment_id) = params.comment_id {
        let _ = match LikeService::unlike_comment(&mm.db(), ctx.user_id, &comment_id).await {
            Ok(_) => {
                info!("Comment unliked successfully");
                return ApiResponse::success(201, "Comment liked successfully", None);
            }
            Err(err) => {
                error!("Failed to unlike comment: {:?}", err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };
    } else {
        error!("Failed to unlike: Missing post_id or comment_id query params");
        return ApiResponse::error(FAILED_MESSAGE, Error::MissingQuery);
    }
}

#[derive(Deserialize)]
pub struct LikeQuery {
    pub post_id: Option<Uuid>,
    pub comment_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct LikeResponse {
    pub like: LikeRepo,
}

#[derive(Serialize)]
pub struct LikeCountResponse {
    pub likes_count: usize,
}

#[derive(Serialize)]
pub struct IsLikedResponse {
    pub is_liked: bool,
}
