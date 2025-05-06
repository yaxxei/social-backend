use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::{error::Error, utils::response::ApiResponse};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = JsonValidationError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JsonValidationError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),
}

impl IntoResponse for JsonValidationError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            _ => ApiResponse::<()>::error("Validation error", Error::JsonValidation(self)),
        }
        .into_response()
    }
}
