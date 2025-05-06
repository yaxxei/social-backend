use axum::{http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Serialize;

use crate::error::Error;

#[derive(Serialize)]
pub struct ApiSuccess<T> {
    status: u16,
    message: String,
    data: Option<T>,
}

#[derive(Serialize)]
pub struct ApiError {
    status: u16,
    message: String,
    error: Option<String>,
}

pub enum ApiResponse<T> {
    Success(ApiSuccess<T>),
    SuccessWithJar(ApiSuccess<T>, CookieJar),
    Error(ApiError),
}

impl<T> ApiResponse<T> {
    pub fn success(status: u16, message: &str, data: Option<T>) -> Self {
        ApiResponse::Success(ApiSuccess {
            status,
            message: message.to_string(),
            data,
        })
    }

    pub fn success_with_jar(status: u16, message: &str, data: Option<T>, jar: CookieJar) -> Self {
        ApiResponse::SuccessWithJar(
            ApiSuccess {
                status,
                message: message.to_string(),
                data,
            },
            jar,
        )
    }

    pub fn error(message: &str, error: Error) -> Self {
        ApiResponse::Error(ApiError {
            status: error.status_code(),
            message: message.to_string(),
            error: Some(error.to_string()),
        })
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponse::Success(success) => {
                let status_code = StatusCode::from_u16(success.status).unwrap_or(StatusCode::OK);
                (status_code, Json(success)).into_response()
            }
            ApiResponse::SuccessWithJar(success, jar) => (
                jar,
                self::ApiResponse::success(success.status, &success.message, success.data),
            )
                .into_response(),
            ApiResponse::Error(error) => {
                let status_code =
                    StatusCode::from_u16(error.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                (status_code, Json(error)).into_response()
            }
        }
    }
}
