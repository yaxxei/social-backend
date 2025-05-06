use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use lib_core::model::ModelManager;
use lib_web::{
    handlers::{handlers_comment, handlers_post},
    middlewares,
};

pub async fn routes(mm: Arc<ModelManager>) -> Router {
    Router::new()
        .route("/", get(handlers_post::get_posts))
        .route("/", post(handlers_post::create_post))
        .route("/{id}", get(handlers_post::get_post))
        .route("/{id}", put(handlers_post::update_post))
        .route("/{id}", delete(handlers_post::delete_post))
        // comment
        .route("/{id}/comments", get(handlers_comment::get_post_comments))
        .route("/{id}/comments", post(handlers_comment::create_comment))
        .route(
            "/{post_id}/comments/{comment_id}",
            put(handlers_comment::update_comment),
        )
        .route(
            "/{post_id}/comments/{comment_id}",
            delete(handlers_comment::delete_comment),
        )
        .with_state(mm)
        .layer(middleware::from_fn(middlewares::require_auth))
}
