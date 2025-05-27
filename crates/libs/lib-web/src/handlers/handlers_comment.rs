use std::sync::Arc;

use axum::extract::{Path, Query, State};
use futures::SinkExt as _;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use validator::Validate;

use crate::{
    extractors::{CtxExt, ValidatedJson},
    services::{
        comment_service::{CommentDto, CommentService},
        post_service::{PostDto, PostService},
    },
    utils::response::ApiResponse,
};

use super::AppState;

pub async fn get_post_comments(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(post_id): Path<Uuid>,
    Query(_): Query<CommentQuery>,
) -> ApiResponse<CommentsResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch comments";
    info!("Starting fetch comments");

    let comments =
        match CommentService::get_many_by_post_id(state.mm.db(), ctx.user_id, &post_id).await {
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
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<CommentQuery>,
) -> ApiResponse<CommentsResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch comments";
    info!("Starting fetch comments");

    let comments = if let Some(user_id) = params.user_id {
        // Получение комментариев конкретного пользователя
        match CommentService::get_many_by_user_id(state.mm.db(), ctx.user_id, &user_id).await {
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
        match CommentService::get_many_by_post_id(state.mm.db(), ctx.user_id, &post_id).await {
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
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<CommentResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch comment";
    info!("Starting fetch comment by id: {}", id);

    let comment = match CommentService::get_by_id(state.mm.db(), ctx.user_id, &id).await {
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

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum OutgoingWsMessage {
    #[serde(rename = "new_post")]
    NewComment { post: PostDto, comment: CommentDto },
}

pub async fn create_comment(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(payload): ValidatedJson<CreateCommentPayload>,
) -> ApiResponse<CommentResponse> {
    const FAILED_MESSAGE: &str = "Failed to create comment";
    info!("Starting create comment by user: {:?}", ctx.user_id);

    let comment = match CommentService::create(
        state.mm.db(),
        ctx.user_id,
        &payload.post_id,
        payload.parent_comment_id,
        &payload.content,
    )
    .await
    {
        Ok(comment) => {
            info!("Comment created: {}", comment.id);

            let post =
                match PostService::get_by_id(state.mm.db(), ctx.user_id, &comment.post_id).await {
                    Ok(post) => post,
                    Err(err) => {
                        error!(
                            "Failed to fetch post {} for comment {}: {}",
                            comment.post_id, comment.id, err
                        );
                        return ApiResponse::error(FAILED_MESSAGE, err);
                    }
                };

            let mut conns = state.notification_conns.lock().await;
            if let Some(sender) = conns.get_mut(&post.user_id) {
                let notif = OutgoingWsMessage::NewComment {
                    comment: comment.clone(),
                    post: post.clone(),
                };

                match serde_json::to_string(&notif) {
                    Ok(json) => {
                        if sender
                            .send(axum::extract::ws::Message::Text(json.into()))
                            .await
                            .is_ok()
                        {
                            debug!(
                    "Sent new comment notification to post author {} (user {}) for post {} (comment {})",
                    post.user_id, post.user_id, post.id, comment.id
                );
                        } else {
                            warn!(
                    "Failed to send comment notification to post author {} (websocket error)",
                    post.user_id
                );
                        }
                    }
                    Err(e) => {
                        warn!(
                            "Failed to serialize comment notification for post author {}: {}",
                            post.user_id, e
                        );
                    }
                }
            } else {
                debug!(
                    "No active connection for post author {}, skipping comment {} notification",
                    post.user_id, comment.id
                );
            }

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
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateCommentPayload>,
) -> ApiResponse<CommentResponse> {
    const FAILED_MESSAGE: &str = "Failed to update comment";
    info!("Starting update comment by user: {:?}", ctx.user_id);

    let comment =
        match CommentService::update(state.mm.db(), ctx.user_id, &id, payload.content).await {
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
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to delete comment";
    info!("Starting delete comment by user: {:?}", ctx.user_id);

    match CommentService::delete(state.mm.db(), ctx.user_id, &id).await {
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
