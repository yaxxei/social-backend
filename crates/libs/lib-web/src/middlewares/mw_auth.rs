use crate::error::Error;
use crate::error::Result;
use crate::utils::response::ApiResponse;
use axum::{extract::Request, middleware::Next, response::IntoResponse};
use lib_auth::token::{verify_token, TokenType};
use tracing::info;
use tracing::warn;
use uuid::Uuid;

pub async fn require_auth(mut req: Request, next: Next) -> impl IntoResponse {
    info!("Access checking by request");

    let auth_result = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|token| validate_token(token));

    match auth_result {
        Some(Ok(user_id)) => {
            info!("Access allowed for: {}", user_id);
            req.extensions_mut().insert(Some(user_id));
            next.run(req).await
        }
        Some(Err(e)) => {
            warn!("Request canceled: Invlid token");
            ApiResponse::<()>::error("Invalid token", e).into_response()
        }
        None => {
            info!("No valid auth provided, continuing as guest");
            req.extensions_mut().insert(None::<Uuid>);
            next.run(req).await
        }
    }
}

fn validate_token(token: &str) -> Result<Uuid> {
    let token_data = verify_token(token, TokenType::Access).map_err(|_| Error::Unauthorized)?;
    Uuid::parse_str(&token_data.claims.sub).map_err(|_| Error::Unauthorized)
}
