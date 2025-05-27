use std::sync::Arc;

use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{
    extractors::CtxExt,
    services::{
        community_service::{CommunityDto, CommunityService},
        post_service::{PostDto, PostService},
        user_service::{UserDto, UserService},
    },
    utils::response::ApiResponse,
};

use super::AppState;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

pub async fn search(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<SearchQuery>,
) -> ApiResponse<SearchResponse> {
    const FAILED_MESSAGE: &str = "Failed to search";
    info!("Starting search for user: {:?}", ctx.user_id);

    let users =
        match UserService::get_meny_by_query(state.mm.db(), ctx.user_id, &params.query).await {
            Ok(users) => users,
            Err(e) => {
                error!("Error while searching users");
                return ApiResponse::error(FAILED_MESSAGE, e);
            }
        };

    let communities = match CommunityService::get_meny_by_query(
        state.mm.db(),
        ctx.user_id,
        &params.query,
    )
    .await
    {
        Ok(communities) => communities,
        Err(e) => {
            error!("Error while searching communities");
            return ApiResponse::error(FAILED_MESSAGE, e);
        }
    };

    let posts =
        match PostService::get_meny_by_query(state.mm.db(), ctx.user_id, &params.query).await {
            Ok(posts) => posts,
            Err(e) => {
                error!("Error while searching posts");
                return ApiResponse::error(FAILED_MESSAGE, e);
            }
        };

    let search_response = SearchResponse {
        users,
        communities,
        posts,
    };

    info!("Searched successully for user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Searched successully", Some(search_response))
}

#[derive(Serialize)]
pub struct SearchResponse {
    users: Vec<UserDto>,
    communities: Vec<CommunityDto>,
    posts: Vec<PostDto>,
}
