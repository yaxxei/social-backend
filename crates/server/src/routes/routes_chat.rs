use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use lib_web::{
    handlers::{handlers_chat, handlers_messages, AppState},
    middlewares,
};

pub async fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers_chat::get_chats))
        .route("/", post(async move || {}))
        .route("/{id}", get(handlers_chat::get_chat))
        .route("/{id}", delete(async move || {}))
        .route("/{id}", put(async move || {}))
        // messages
        .route("/messages", get(handlers_messages::get_messages))
        .route("/messages", post(handlers_messages::create_message))
        .route("/messages/{id}", get(handlers_messages::get_message))
        .route("/messages/{id}", put(handlers_messages::update_message))
        .route("/messages/{id}", delete(handlers_messages::delete_message))
        .route("/messages/{id}/read", post(handlers_messages::read_message))
        .with_state(state)
        .layer(middleware::from_fn(middlewares::require_auth))
}
