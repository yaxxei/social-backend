use std::sync::Arc;

use axum::extract::{Path, Query, State};
use lib_core::model::ModelManager;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

use crate::{
    extractors::{CtxExt, ValidatedJson},
    services::comment_service::{CommentDto, CommentService},
    utils::response::ApiResponse,
};

pub async fn get_post_comments(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(post_id): Path<Uuid>,
    Query(_): Query<CommentQuery>,
) -> ApiResponse<CommentsResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch comments";
    info!("Starting fetch comments");

    let comments = match CommentService::get_many_by_post_id(mm.db(), ctx.user_id, &post_id).await {
        Ok(comments) => {
            info!(
                "Successfully fetched {} comments for post: {}",
                comments.len(),
                post_id
            );
            comments
        }
        Err(err) => {
            error!("Failed to fetch comments for post {}: {:?}", post_id, err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let comments_response = CommentsResponse { comments };

    info!("Comments fetched successfully for post: {}", post_id);
    ApiResponse::success(
        200,
        "Comments fetched successfully",
        Some(comments_response),
    )
}

pub async fn get_comments(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<CommentQuery>,
) -> ApiResponse<CommentsResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch comments";
    info!("Starting fetch comments");

    let comments = if let Some(user_id) = params.user_id {
        // Получение комментариев конкретного пользователя
        match CommentService::get_many_by_user_id(mm.db(), ctx.user_id, &user_id).await {
            Ok(comments) => {
                info!(
                    "Successfully fetched {} comments for user: {}",
                    comments.len(),
                    user_id
                );
                comments
            }
            Err(err) => {
                error!("Failed to fetch comments for user {}: {:?}", user_id, err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        }
    } else if let Some(post_id) = params.post_id {
        // Получение комментариев для конкретного поста
        match CommentService::get_many_by_post_id(mm.db(), ctx.user_id, &post_id).await {
            Ok(comments) => {
                info!(
                    "Successfully fetched {} comments for post: {}",
                    comments.len(),
                    post_id
                );
                comments
            }
            Err(err) => {
                error!("Failed to fetch comments for post {}: {:?}", post_id, err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        }
    } else {
        error!("Null query params");
        Vec::<CommentDto>::new()
    };

    let comments_response = CommentsResponse { comments };

    info!("Comments fetched successfully");
    ApiResponse::success(
        200,
        "Comments fetched successfully",
        Some(comments_response),
    )
}

pub async fn get_comment(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<CommentResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch comment";
    info!("Starting fetch comment by id: {}", id);

    let comment = match CommentService::get_by_id(mm.db(), ctx.user_id, &id).await {
        Ok(comment) => {
            info!("Comment found: {}", id);
            comment
        }
        Err(err) => {
            error!("Failed to find comment by id: {}", id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let comment_response = CommentResponse { comment };

    info!("Comment fetched successfully by id: {}", id);
    ApiResponse::success(200, "Comment fetched successfully", Some(comment_response))
}

pub async fn create_comment(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(payload): ValidatedJson<CreateCommentPayload>,
) -> ApiResponse<CommentResponse> {
    const FAILED_MESSAGE: &str = "Failed to create comment";
    info!("Starting create comment by user: {:?}", ctx.user_id);

    let comment = match CommentService::create(
        mm.db(),
        ctx.user_id,
        &payload.post_id,
        payload.parent_comment_id,
        &payload.content,
    )
    .await
    {
        Ok(comment) => {
            info!("Comment created: {}", comment.id);
            comment
        }
        Err(err) => {
            error!("Failed to create comment by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let comment_response = CommentResponse { comment };

    info!("Comment created successfully by user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Comment created successfully", Some(comment_response))
}

pub async fn update_comment(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateCommentPayload>,
) -> ApiResponse<CommentResponse> {
    const FAILED_MESSAGE: &str = "Failed to update comment";
    info!("Starting update comment by user: {:?}", ctx.user_id);

    let comment = match CommentService::update(mm.db(), ctx.user_id, &id, payload.content).await {
        Ok(comment) => {
            info!("Comment updated: {}", comment.id);
            comment
        }
        Err(err) => {
            error!("Failed to update comment by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let comment_response = CommentResponse { comment };

    info!("Comment updated successfully by user: {:?}", ctx.user_id);
    ApiResponse::success(200, "Comment updated successfully", Some(comment_response))
}

pub async fn delete_comment(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to delete comment";
    info!("Starting delete comment by user: {:?}", ctx.user_id);

    match CommentService::delete(mm.db(), ctx.user_id, &id).await {
        Ok(_) => {
            info!("Comment deleted: {}", id);
        }
        Err(err) => {
            error!("Failed to delete comment by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    info!("Comment deleted successfully by user: {:?}", ctx.user_id);
    ApiResponse::success(200, "Comment deleted successfully", None)
}

#[derive(Deserialize)]
pub struct CommentQuery {
    pub user_id: Option<Uuid>,
    pub post_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentPayload {
    pub post_id: Uuid,
    #[validate(length(min = 1))]
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCommentPayload {
    #[validate(length(min = 1))]
    pub content: Option<String>,
}

#[derive(Serialize)]
pub struct CommentResponse {
    pub comment: CommentDto,
}

#[derive(Serialize)]
pub struct CommentsResponse {
    pub comments: Vec<CommentDto>,
}
