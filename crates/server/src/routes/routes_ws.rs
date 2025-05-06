use std::sync::Arc;

use axum::{routing::any, Router};
use lib_web::handlers::{handlers_notification, ws_handlers_chat, AppState};

pub async fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/chat/{id}", any(ws_handlers_chat::ws_handler))
        .route(
            "/notifications",
            any(handlers_notification::notification_ws_handler),
        )
        .with_state(state)
}
