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
        .route("/", post(handlers_chat::create_chat))
        .route("/{id}", get(handlers_chat::get_chat))
        .route("/{id}", put(handlers_chat::update_chat))
        .route("/{id}", delete(handlers_chat::delete_chat))
        .route("/{id}/get-members", get(handlers_chat::get_members))
        .route(
            "/{id}/add-user",
            post(handlers_chat::add_user_to_group_chat),
        )
        .route(
            "/{id}/remove-user",
            post(handlers_chat::remove_user_from_group_chat),
        )
        .route("/has-chat", get(handlers_chat::has_chats_with_user))
        // search
        .route("/search", get(handlers_chat::search))
        // messages
        .route("/messages", get(handlers_messages::get_messages))
        .route("/messages", post(handlers_messages::create_message))
        .route("/messages/read", post(handlers_messages::read_messages))
        .route("/messages/{id}", get(handlers_messages::get_message))
        .route("/messages/{id}", put(handlers_messages::update_message))
        .route("/messages/{id}", delete(handlers_messages::delete_message))
        .route("/messages/{id}/read", post(handlers_messages::read_message))
        .with_state(state)
        .layer(middleware::from_fn(middlewares::require_auth))
}
