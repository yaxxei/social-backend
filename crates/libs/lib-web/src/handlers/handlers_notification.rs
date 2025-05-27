use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket},
        Query, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::StreamExt as _;
use serde::Deserialize;
use tracing::debug;
use uuid::Uuid;

use super::AppState;

#[derive(Deserialize)]
pub struct NotificationQuery {
    user_id: Uuid,
}

pub async fn notification_ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<NotificationQuery>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, params.user_id, state))
}

async fn handle_socket(socket: WebSocket, user_id: Uuid, state: Arc<AppState>) {
    let (sender, mut receiver) = socket.split();

    let _ = state
        .notification_conns
        .lock()
        .await
        .insert(user_id, sender);
    debug!("User {} connected to notification", user_id);

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            WsMessage::Close(_) => break,
            _ => {}
        }
    }

    let _ = state.notification_conns.lock().await.remove(&user_id);
}
