use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use lib_web::{
    handlers::{handlers_comment, AppState},
    middlewares,
};

pub async fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers_comment::get_comments))
        .route("/", post(handlers_comment::create_comment))
        .route("/{id}", get(handlers_comment::get_comment))
        .route("/{id}/thread", get(handlers_comment::get_comment_thread))
        .route("/{id}", put(handlers_comment::update_comment))
        .route("/{id}", delete(handlers_comment::delete_comment))
        .with_state(state)
        .layer(middleware::from_fn(middlewares::require_auth))
}
