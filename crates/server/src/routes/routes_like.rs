use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, post},
    Router,
};
use lib_web::{
    handlers::{handlers_like, AppState},
    middlewares,
};

pub async fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/like", post(handlers_like::like))
        .route("/dislike", post(handlers_like::dislike))
        .route("/unlike", delete(handlers_like::unlike))
        .with_state(state)
        .layer(middleware::from_fn(middlewares::require_auth))
}
