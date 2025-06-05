use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use lib_web::{
    handlers::{handlers_report, AppState},
    middlewares,
};

pub async fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers_report::get_reports))
        .route("/", post(handlers_report::create_report))
        .route("/{id}", get(handlers_report::get_report))
        .route("/{id}", put(handlers_report::update_report_status))
        .route("/{id}", delete(handlers_report::delete_report))
        .with_state(state)
        .layer(middleware::from_fn(middlewares::require_auth))
}
