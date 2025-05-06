#![allow(warnings)]

use axum::extract::{Path, Query, State};
use lib_core::{
    ctx::Ctx,
    model::{
        user::{self},
        ModelManager,
    },
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, error, info};
use uuid::Uuid;
use validator::Validate;

use crate::extractors::{CtxExt, ValidatedJson};
use crate::services::user_service::{UserDto, UserService};
use crate::utils::response::ApiResponse;

pub async fn get_all_users(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
) -> ApiResponse<UsersResponse> {
    const FAILED_MESSAGE: &str = "Failed to register";
    info!("Starting fetching users");

    let users = match UserService::get_all(mm.db(), ctx.user_id).await {
        Ok(users) => {
            info!("Users fetched successul");
            users
        }
        Err(err) => {
            error!("Failed to fetch users");
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let users_response = UsersResponse { users };

    info!("Fetching users successful");
    ApiResponse::success(200, "Users fetched successully", Some(users_response))
}

pub async fn get_user_profile(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(nickname): Path<String>,
) -> ApiResponse<UserResponse> {
    const FAILED_MESSAGE: &str = "Failed to find user";
    info!("Starting fetching user");

    let user = match UserService::get_by_nickname(mm.db(), ctx.user_id, &nickname).await {
        Ok(user) => {
            debug!("User found: {}", user.nickname);
            user
        }
        Err(err) => {
            error!("Failed to find user: {}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let user_response = UserResponse { user };

    info!("Fetching user successful");
    ApiResponse::success(200, "User fetched successully", Some(user_response))
}

pub async fn update_user_profile(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(nickname): Path<String>,
    ValidatedJson(payload): ValidatedJson<UserUpdatePayload>,
) -> ApiResponse<UserResponse> {
    const FAILED_MESSAGE: &str = "Failed to update user";
    info!("Starting udpate user");

    let user = match UserService::get_by_nickname(mm.db(), ctx.user_id, &nickname).await {
        Ok(user) => {
            debug!("User found: {}", user.nickname);
            user
        }
        Err(err) => {
            error!("Failed to find user: {}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let user = match UserService::update(
        mm.db(),
        ctx.user_id,
        &user.id,
        payload.nickname,
        payload.email,
        None,
        None,
    )
    .await
    {
        Ok(user) => {
            debug!("User updated: {}", user.id);
            user
        }
        Err(err) => {
            error!("Failed to update user: {}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let user_response = UserResponse { user };

    info!("Updating user successful");
    ApiResponse::success(200, "User updated successully", Some(user_response))
}

pub async fn delete_user_profile(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
    Path(nickname): Path<String>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to delete user";
    info!("Starting delete user");

    let user = match UserService::get_by_nickname(mm.db(), ctx.user_id, &nickname).await {
        Ok(user) => {
            debug!("User found: {}", user.nickname);
            user
        }
        Err(err) => {
            error!("Failed to find user: {}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let _ = match UserService::delete(mm.db(), ctx.user_id, &user.id).await {
        Ok(_) => {
            debug!("User deleted: {}", user.id);
        }
        Err(err) => {
            error!("Failed to delete user: {}", err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    info!("Deleting user successful");
    ApiResponse::success(200, "User deleted successully", None)
}

#[derive(Deserialize, Validate)]
pub struct UserUpdatePayload {
    #[validate(length(
        min = 1,
        max = 16,
        message = "Nickname must contain at least 1 characters and no more than 16"
    ))]
    nickname: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    email: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse {
    user: UserDto,
}

#[derive(Serialize)]
pub struct UsersResponse {
    users: Vec<UserDto>,
}
