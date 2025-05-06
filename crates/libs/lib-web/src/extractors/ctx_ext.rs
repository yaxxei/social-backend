use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::IntoResponse;
use lib_core::ctx::Ctx;
use std::future::Future;
use uuid::Uuid;

use crate::{error::Error, utils::response::ApiResponse};

impl<S> FromRequestParts<S> for CtxExt
where
    S: Send + Sync,
{
    type Rejection = CtxExtError;

    fn from_request_parts(
        parts: &mut Parts,
        _: &S,
    ) -> impl Future<Output = core::result::Result<Self, Self::Rejection>> + Send {
        async move {
            let user_id = parts
                .extensions
                .get::<Option<Uuid>>()
                .ok_or(CtxExtError::CannotExtractContext)?;

            Ok(CtxExt(Ctx { user_id: *user_id }))
        }
    }
}

pub struct CtxExt(pub Ctx);

#[derive(Debug, thiserror::Error)]
pub enum CtxExtError {
    #[error(transparent)]
    CtxError(#[from] lib_core::ctx::error::Error),

    #[error("Cannot extract current user id from request")]
    CannotExtractContext,
}

impl IntoResponse for CtxExtError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            _ => ApiResponse::<()>::error("Access denied", Error::CtxExt(self)),
        }
        .into_response()
    }
}
