use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use lib_core::model::ModelManager;
use lib_web::{
    handlers::{handlers_community, handlers_follow},
    middlewares,
};

pub async fn routes(mm: Arc<ModelManager>) -> Router {
    Router::new()
        .route("/", get(handlers_community::get_communities))
        .route("/", post(handlers_community::create))
        .route("/{name}", get(handlers_community::get))
        .route("/{name}", delete(handlers_community::delete))
        .route("/{name}", put(handlers_community::update))
        // follows
        .route("/{id}/follow", post(handlers_follow::follow))
        .route("/{id}/unfollow", delete(handlers_follow::unfollow))
        .route("/{id}/follow/check", get(handlers_follow::is_followd))
        .layer(middleware::from_fn(middlewares::require_auth))
        .with_state(mm)
}
