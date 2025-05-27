use std::sync::Arc;

use axum::{middleware, routing::get, Router};
use lib_web::{
    handlers::{handlers_search, AppState},
    middlewares,
};

pub async fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers_search::search))
        .with_state(state)
        .layer(middleware::from_fn(middlewares::require_auth))
}
