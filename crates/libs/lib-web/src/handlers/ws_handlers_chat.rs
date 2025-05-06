#![allow(unused)]

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket},
        ConnectInfo, Path, Query, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{SinkExt as _, StreamExt as _};
use lib_core::ctx::Ctx;
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, error};
use uuid::Uuid;

use crate::services::chat_service::{ChatService, MessageDto};

use crate::{error::Error, extractors::CtxExt, utils::response::ApiResponse};

use super::{AppState, UserConnection};

#[derive(Deserialize)]
pub struct WsChatQuery {
    user_id: Uuid,
}

pub async fn ws_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Query(params): Query<WsChatQuery>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_chat_socket(socket, addr, state, id, params.user_id))
}

pub async fn handle_chat_socket(
    socket: WebSocket,
    _: SocketAddr,
    state: Arc<AppState>,
    chat_id: Uuid,
    user_id: Uuid,
) {
    let (mut sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<WsMessage>();

    {
        let mut conns = state.chat_conns.lock().await;
        conns.entry(chat_id).or_default().push(UserConnection {
            user_id,
            sender: tx,
        });
    }

    debug!("User {user_id} connected to chat {chat_id}");

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                debug!("Send to user {user_id} failed — client disconnected");
                break;
            };
        }
    });

    let state_clone = Arc::clone(&state);
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(WsMessage::Text(msg))) = receiver.next().await {
            match serde_json::from_str(msg.as_str()) {
                Ok(IncomingWsMessage::SendMessage { chat_id, content }) => {
                    match ChatService::send_message(
                        state_clone.mm.clone(),
                        Ctx::new(user_id),
                        &chat_id,
                        &content,
                    )
                    .await
                    {
                        Ok(msg) => {
                            debug!("Sending message to all users in chat {:?}", msg);

                            let conns = state_clone.chat_conns.lock().await;
                            if let Some(users) = conns.get(&chat_id) {
                                for conn in users {
                                    if conn.user_id != user_id {
                                        let outgoing = OutgoingWsMessage::NewMessage {
                                            message: msg.clone(),
                                        };
                                        if let Ok(payload) = serde_json::to_string(&outgoing) {
                                            if let Err(err) =
                                                conn.sender.send(WsMessage::Text(payload.into()))
                                            {
                                                error!(
                                                    "Failed to send message to user {}: {:?}",
                                                    conn.user_id, err
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => error!("Message sending error: {:?}", err),
                    }
                }
                Err(err) => error!("Json parsing error: {:?}", err),
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    {
        let mut conns = state.chat_conns.lock().await;
        if let Some(users) = conns.get_mut(&chat_id) {
            users.retain(|conn| conn.user_id != user_id);
        }
    }

    debug!("User {user_id} disconnected from chat {chat_id}")
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum IncomingWsMessage {
    #[serde(rename = "send_message")]
    SendMessage { chat_id: Uuid, content: String },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum OutgoingWsMessage {
    #[serde(rename = "new_message")]
    NewMessage { message: MessageDto },
}
