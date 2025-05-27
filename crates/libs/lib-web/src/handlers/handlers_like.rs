use axum::extract::{Query, State};
use futures::SinkExt as _;
use lib_core::model::like::LikeRepo;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::Error,
    extractors::CtxExt,
    services::{
        comment_service::{CommentDto, CommentService},
        like_service::LikeService,
        post_service::{PostDto, PostService},
        user_service::{UserDto, UserService},
    },
    utils::response::ApiResponse,
};
use tracing::{debug, error, info, warn};

use super::AppState;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum OutgoingWsMessage {
    #[serde(rename = "new_like")]
    NewPostLike { post: PostDto, user: UserDto },

    #[serde(rename = "new_like")]
    NewCommentLike { comment: CommentDto, user: UserDto },
}

pub async fn like(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<LikeQuery>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to like";
    info!("Starting like");

    let requester_id = match ctx.user_id.ok_or(Error::Unauthorized) {
        Ok(uid) => uid,
        Err(e) => {
            error!("Requester Unauthorized");
            return ApiResponse::error(FAILED_MESSAGE, e);
        }
    };

    if let Some(post_id) = params.post_id {
        let _ = match LikeService::like_post(&state.mm.db(), ctx.user_id, &post_id, 1).await {
            Ok(_) => {
                info!("Post liked successfully");

                match PostService::get_by_id(state.mm.db(), ctx.user_id, &post_id).await {
                    Ok(post) => {
                        debug!("Retrieved post {} for like notification", post.id);

                        if let Some(sender) =
                            state.notification_conns.lock().await.get_mut(&post.user_id)
                        {
                            let user = match UserService::get_by_id(
                                state.mm.db(),
                                ctx.user_id,
                                &requester_id,
                            )
                            .await
                            {
                                Ok(user) => user,
                                Err(err) => {
                                    warn!(
                                        "Failed to fetch user {} info for like notification: {}",
                                        requester_id, err
                                    );
                                    return ApiResponse::error(FAILED_MESSAGE, err);
                                }
                            };

                            let notif = OutgoingWsMessage::NewPostLike {
                                post: post.clone(),
                                user: user.clone(),
                            };

                            match serde_json::to_string(&notif) {
                                Ok(json) => {
                                    if sender
                                        .send(axum::extract::ws::Message::Text(json.into()))
                                        .await
                                        .is_ok()
                                    {
                                        debug!(
                            "Successfully sent post like notification to post author {} about like from user {}",
                            post.user_id, requester_id
                        );
                                    } else {
                                        warn!(
                            "Failed to send post like notification to post author {} (websocket send failed)",
                            post.user_id
                        );
                                    }
                                }
                                Err(e) => {
                                    warn!(
                                        "Failed to serialize like notification for post {}: {}",
                                        post.id, e
                                    );
                                }
                            }
                        } else {
                            debug!(
                "No active websocket connection for post author {}, skipping like notification",
                post.user_id
            );
                        }
                    }
                    Err(err) => {
                        warn!(
                            "Failed to retrieve post {} for like notification: {}",
                            post_id, err
                        );
                    }
                }

                return ApiResponse::success(201, "Post liked successfully", None);
            }
            Err(err) => {
                error!("Failed to like post: {:?}", err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };
    } else if let Some(comment_id) = params.comment_id {
        let _ = match LikeService::like_comment(&state.mm.db(), ctx.user_id, &comment_id, 1).await {
            Ok(_) => {
                info!("Comment liked successfully");

                match CommentService::get_by_id(state.mm.db(), ctx.user_id, &comment_id).await {
                    Ok(comment) => {
                        debug!("Retrieved comment {} for like notification", comment.id);

                        if let Some(sender) = state
                            .notification_conns
                            .lock()
                            .await
                            .get_mut(&comment.user_id)
                        {
                            // Получаем информацию о пользователе, который поставил лайк
                            let liker = match UserService::get_by_id(
                                state.mm.db(),
                                ctx.user_id,
                                &requester_id,
                            )
                            .await
                            {
                                Ok(user) => user,
                                Err(err) => {
                                    warn!("Failed to fetch liker {} info for comment like notification: {}", 
                        requester_id, err);
                                    return ApiResponse::error(FAILED_MESSAGE, err);
                                }
                            };

                            let notif = OutgoingWsMessage::NewCommentLike {
                                comment: comment.clone(),
                                user: liker.clone(),
                            };

                            match serde_json::to_string(&notif) {
                                Ok(json) => {
                                    if sender
                                        .send(axum::extract::ws::Message::Text(json.into()))
                                        .await
                                        .is_ok()
                                    {
                                        debug!(
                            "Sent comment like notification to author {} about like from user {} for comment {}",
                            comment.user_id, requester_id, comment.id
                        );
                                    } else {
                                        warn!(
                            "Failed to send comment like notification to author {} (websocket error)",
                            comment.user_id
                        );
                                    }
                                }
                                Err(e) => {
                                    warn!(
                        "Failed to serialize comment like notification for comment {}: {}",
                        comment.id, e
                    );
                                }
                            }
                        } else {
                            debug!(
                "No active websocket connection for comment author {}, skipping like notification",
                comment.user_id
            );
                        }
                    }
                    Err(err) => {
                        warn!(
                            "Failed to retrieve comment {} for like notification: {}",
                            comment_id, err
                        );
                    }
                }

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
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<LikeQuery>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to dislike";
    info!("Starting dislike");

    if let Some(post_id) = params.post_id {
        let _ = match LikeService::like_post(&state.mm.db(), ctx.user_id, &post_id, -1).await {
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
        let _ = match LikeService::like_comment(&state.mm.db(), ctx.user_id, &comment_id, -1).await
        {
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
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<LikeQuery>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to unlike";
    info!("Starting unlike");

    if let Some(post_id) = params.post_id {
        let _ = match LikeService::unlike_post(&state.mm.db(), ctx.user_id, &post_id).await {
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
        let _ = match LikeService::unlike_comment(&state.mm.db(), ctx.user_id, &comment_id).await {
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
