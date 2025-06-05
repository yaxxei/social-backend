use std::sync::Arc;

use axum::extract::{Path, Query, State};
use futures::SinkExt as _;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::Error,
    extractors::{CtxExt, ValidatedJson},
    services::{
        follow_service::FollowService,
        post_service::{PostDto, PostService},
    },
    utils::response::ApiResponse,
};

use super::AppState;

pub async fn get_posts(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<PostQuery>,
) -> ApiResponse<PostsResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch posts";
    info!("Starting fetch posts");

    let posts = if let Some(user_id) = params.user_id {
        match PostService::get_many_by_user_id(state.mm.db(), ctx.user_id, &user_id).await {
            Ok(posts) => {
                info!(
                    "Successfully fetched {} posts for user: {}",
                    posts.len(),
                    user_id
                );
                posts
            }
            Err(err) => {
                error!("Failed to fetch posts for user {}: {:?}", user_id, err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        }
    } else if let Some(community_id) = params.community_id {
        match PostService::get_many_by_community_id(state.mm.db(), ctx.user_id, &community_id).await
        {
            Ok(posts) => {
                info!(
                    "Successfully fetched {} posts for community: {}",
                    posts.len(),
                    community_id
                );
                posts
            }
            Err(err) => {
                error!(
                    "Failed to fetch posts for community {}: {:?}",
                    community_id, err
                );
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        }
    } else {
        match PostService::get_many(state.mm.db(), ctx.user_id).await {
            Ok(posts) => {
                info!("Successfully fetched {} posts", posts.len(),);
                posts
            }
            Err(err) => {
                error!("Failed to fetch posts: {:?}", err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        }
    };

    let posts_response = PostsResponse { posts };

    info!("Posts fetched successfully");
    ApiResponse::success(200, "Posts fetched successully", Some(posts_response))
}

pub async fn get_post(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<PostResposnse> {
    const FAILED_MESSAGE: &str = "Failed to fetch post";
    info!("Starting fetch post by id: {}", id);

    let post = match PostService::get_by_id(state.mm.db(), ctx.user_id, &id).await {
        Ok(post) => {
            info!("Post found: ");
            post
        }
        Err(err) => {
            error!("Failed to find post by id: {}", id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let post_response = PostResposnse { post };

    info!("Post fetched successully by id: {}", id);
    ApiResponse::success(200, "Post fetched successully", Some(post_response))
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum OutgoingWsMessage {
    #[serde(rename = "new_post")]
    NewPost { post: PostDto },
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(payload): ValidatedJson<CreatePostPayload>,
) -> ApiResponse<PostResposnse> {
    const FAILED_MESSAGE: &str = "Failed to create post";
    info!("Starting create post by user: {:?}", ctx.user_id);

    let requester_id = match ctx.user_id.ok_or(Error::Unauthorized) {
        Ok(uid) => uid,
        Err(e) => {
            error!("Requester Unauthorized");
            return ApiResponse::error(FAILED_MESSAGE, e);
        }
    };

    let post = match PostService::create(
        state.mm.db(),
        ctx.user_id,
        &payload.community_id,
        &payload.title,
        &payload.content,
    )
    .await
    {
        Ok(post) => {
            info!("Post created: {}", post.id);

            if let Ok(uids) =
                FollowService::get_followers(state.mm.db(), ctx.user_id, &post.community_id).await
            {
                for uid in uids
                    .into_iter()
                    .filter(|f| f.user_id.ne(&requester_id))
                    .map(|f| f.user_id)
                {
                    if let Some(sender) = state.notification_conns.lock().await.get_mut(&uid) {
                        let notif = OutgoingWsMessage::NewPost { post: post.clone() };

                        match serde_json::to_string(&notif) {
                            Ok(json) => {
                                if sender
                                    .send(axum::extract::ws::Message::Text(json.into()))
                                    .await
                                    .is_ok()
                                {
                                    debug!(
                                        "Successfully sent new_post notification to user {}",
                                        uid
                                    );
                                } else {
                                    warn!("Failed to send new_post notification to user {} - send error", uid);
                                }
                            }
                            Err(e) => {
                                warn!("Failed to serialize notification for user {}: {}", uid, e);
                            }
                        }
                    } else {
                        warn!("No active websocket connection found for user {}", uid);
                    }
                }
            } else {
                warn!(
                    "Failed to get followers for community {} when posting {}",
                    post.community_id, post.id
                );
            }

            post
        }
        Err(err) => {
            error!("Failed to create post by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let post_response = PostResposnse { post };

    info!("Post created successully by user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Post created successully", Some(post_response))
}

pub async fn update_post(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdatePostPayload>,
) -> ApiResponse<PostResposnse> {
    const FAILED_MESSAGE: &str = "Failed to update post";
    info!("Starting update post by user: {:?}", ctx.user_id);

    let post = match PostService::update(
        state.mm.db(),
        ctx.user_id,
        &id,
        payload.title,
        payload.content,
    )
    .await
    {
        Ok(post) => {
            info!("Post updated: {}", post.id);
            post
        }
        Err(err) => {
            error!("Failed to update post by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let post_response = PostResposnse { post };

    info!("Post updated successully by user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Post updated successully", Some(post_response))
}

pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to delete post";
    info!("Starting delete post by user: {:?}", ctx.user_id);

    let _ = match PostService::delete(state.mm.db(), ctx.user_id, &id).await {
        Ok(post) => {
            info!("Post deleted: {}", &id);
            post
        }
        Err(err) => {
            error!("Failed to delete post by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    info!("Post deleted successully by user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Post deleted successully", None)
}

#[derive(Deserialize)]
pub struct PostQuery {
    user_id: Option<Uuid>,
    community_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostPayload {
    community_id: Uuid,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Title must be between 1 and 100 characters"
    ))]
    title: String,
    content: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePostPayload {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Title must be between 1 and 100 characters"
    ))]
    title: Option<String>,
    content: Option<String>,
}

#[derive(Serialize)]
pub struct PostResposnse {
    post: PostDto,
}

#[derive(Serialize)]
pub struct PostsResponse {
    posts: Vec<PostDto>,
}
