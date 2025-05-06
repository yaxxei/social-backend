use axum::extract::State;
use axum_extra::extract::CookieJar;
use lib_core::model::ModelManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::error;
use validator::Validate;

use crate::extractors::{CtxExt, ValidatedJson};
use crate::services::user_service::UserDto;
use crate::services::user_service::UserService;
use crate::{error::Error, services::auth_service::AuthService, utils::response::ApiResponse};

pub async fn register(
    State(mm): State<Arc<ModelManager>>,
    jar: CookieJar,
    ValidatedJson(payload): ValidatedJson<RegisterPayload>,
) -> ApiResponse<AuthResponse> {
    match AuthService::register(
        mm,
        jar,
        &payload.nickname,
        &payload.email,
        &payload.password,
    )
    .await
    {
        Ok((user, access_token, new_jar)) => ApiResponse::success_with_jar(
            200,
            "Registration successful",
            Some(AuthResponse { user, access_token }),
            new_jar,
        ),
        Err(e) => ApiResponse::error("Failed to register", e),
    }
}

pub async fn login(
    State(mm): State<Arc<ModelManager>>,
    jar: CookieJar,
    ValidatedJson(payload): ValidatedJson<LoginPayload>,
) -> ApiResponse<AuthResponse> {
    match AuthService::login(mm, jar, &payload.nickname, &payload.password).await {
        Ok((user, access_token, new_jar)) => ApiResponse::success_with_jar(
            200,
            "Login successful",
            Some(AuthResponse { user, access_token }),
            new_jar,
        ),
        Err(e) => ApiResponse::error("Failed to login", e),
    }
}

pub async fn logout(State(mm): State<Arc<ModelManager>>, jar: CookieJar) -> ApiResponse<()> {
    match AuthService::logout(mm, jar).await {
        Ok(new_jar) => ApiResponse::success_with_jar(200, "Logout successful", None::<()>, new_jar),
        Err(e) => ApiResponse::error("Failed to logout", e),
    }
}

pub async fn refresh(
    State(mm): State<Arc<ModelManager>>,
    jar: CookieJar,
) -> ApiResponse<AuthResponse> {
    match AuthService::refresh(mm, jar).await {
        Ok((user, access_token, new_jar)) => ApiResponse::success_with_jar(
            200,
            "Refresh successful",
            Some(AuthResponse { user, access_token }),
            new_jar,
        ),
        Err(e) => ApiResponse::error("Failed to refresh", e),
    }
}

pub async fn auth_me(
    State(mm): State<Arc<ModelManager>>,
    CtxExt(ctx): CtxExt,
) -> ApiResponse<UserResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch current user";

    let user = match ctx.user_id {
        Some(id) => match UserService::get_by_id(mm.db(), ctx.user_id, &id).await {
            Ok(user) => user,
            Err(err) => {
                error!("Failed to fetch user: {:?}", err);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        },
        None => {
            error!("Failed to fetch user");
            return ApiResponse::error(FAILED_MESSAGE, Error::Unauthorized);
        }
    };

    ApiResponse::success(
        200,
        "Current user fetched successfully",
        Some(UserResponse { user }),
    )
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(
        min = 1,
        max = 16,
        message = "Nickname must contain at least 1 characters and no more than 16"
    ))]
    nickname: String,

    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordPayload {
    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    old_password: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    new_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(length(
        min = 1,
        max = 16,
        message = "Nickname must contain at least 1 characters and no more than 16"
    ))]
    nickname: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    user: UserDto,
    access_token: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    user: UserDto,
}
