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
use tokio::sync::mpsc;
use tracing::{debug, error, warn};
use uuid::Uuid;

use crate::services::{
    chat_service::{ChatDto, ChatService, MessageDto},
    user_service::UserDto,
};

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

async fn send_to_chat_members(
    state: &Arc<AppState>,
    chat_id: Uuid,
    sender_id: Uuid,
    message: &OutgoingWsMessage,
) {
    let payload = match serde_json::to_string(message) {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to serialize message: {:?}", e);
            return;
        }
    };

    // Получаем участников чата
    let members =
        match ChatService::get_members(state.mm.clone(), Ctx::new(sender_id), &chat_id).await {
            Ok(members) => members,
            Err(e) => {
                error!("Failed to get chat members: {:?}", e);
                return;
            }
        };

    let mut chat_conns = state.chat_conns.lock().await;
    let mut notif_conns = state.notification_conns.lock().await;

    for member in members.into_iter().filter(|user| user.id != sender_id) {
        let uid = member.id;
        let chat_users = chat_conns.get(&chat_id);

        let sent_via_chat = if let Some(users) = chat_users {
            let mut sent = false;
            for conn in users.iter().filter(|c| c.user_id == uid) {
                if conn
                    .sender
                    .send(WsMessage::Text(payload.clone().into()))
                    .is_ok()
                {
                    sent = true;
                }
            }
            sent
        } else {
            false
        };

        if !sent_via_chat {
            if let Some(notif_sender) = notif_conns.get_mut(&uid) {
                if notif_sender
                    .send(WsMessage::Text(payload.clone().into()))
                    .await
                    .is_ok()
                {
                    debug!("Notification message sent to user {}", uid);
                } else {
                    warn!("Failed to send notification message to user {}", uid);
                }
            } else {
                warn!("User {} not connected to notification socket", uid);
            }
        }
    }
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
                            let outgoing = OutgoingWsMessage::NewMessage {
                                message: msg.clone(),
                            };
                            send_to_chat_members(&state_clone, chat_id, user_id, &outgoing).await;
                        }
                        Err(err) => error!("Message sending error: {:?}", err),
                    }
                }
                Ok(IncomingWsMessage::EditMessage {
                    message_id,
                    new_content,
                }) => {
                    match ChatService::update_message(
                        state_clone.mm.clone(),
                        Ctx::new(user_id),
                        &message_id,
                        &new_content,
                    )
                    .await
                    {
                        Ok(msg) => {
                            debug!("Message edited: {:?}", msg);
                            let outgoing = OutgoingWsMessage::MessageEdited {
                                message: msg.clone(),
                            };
                            send_to_chat_members(&state_clone, chat_id, user_id, &outgoing).await;
                        }
                        Err(err) => error!("Message editing error: {:?}", err),
                    }
                }
                Ok(IncomingWsMessage::DeleteMessage { message_id }) => {
                    match ChatService::delete_message(
                        state_clone.mm.clone(),
                        Ctx::new(user_id),
                        &message_id,
                    )
                    .await
                    {
                        Ok(msg) => {
                            debug!("Message deleted: {:?}", msg);
                            let outgoing = OutgoingWsMessage::MessageDeleted {
                                message: msg.clone(),
                            };
                            send_to_chat_members(&state_clone, chat_id, user_id, &outgoing).await;
                        }
                        Err(err) => error!("Message deletion error: {:?}", err),
                    }
                }
                Ok(IncomingWsMessage::UserRemoved { chat, user }) => {
                    match ChatService::get_chat_owner(
                        state_clone.mm.clone(),
                        Ctx::new(user_id),
                        &chat_id,
                    )
                    .await
                    {
                        Ok(owner) => {
                            if owner.id == user.id {
                                let mut conns = state_clone.chat_conns.lock().await;
                                if let Some(users) = conns.get_mut(&chat_id) {
                                    users.clear();
                                }
                                debug!(
                                    "Owner {} removed, disconnected all from chat {}",
                                    user.id, chat_id
                                );
                            } else {
                                let mut conns = state_clone.chat_conns.lock().await;
                                if let Some(users) = conns.get_mut(&chat_id) {
                                    users.retain(|conn| conn.user_id != user.id);
                                }
                                debug!("User {} removed from chat {}", user.id, chat_id);
                            }
                        }
                        Err(e) => {
                            error!("Failed to get chat owner: {:?}", e);
                        }
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

    debug!("User {user_id} disconnected from chat {chat_id}");
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum IncomingWsMessage {
    #[serde(rename = "send_message")]
    SendMessage { chat_id: Uuid, content: String },

    #[serde(rename = "edit_message")]
    EditMessage {
        message_id: Uuid,
        new_content: String,
    },

    #[serde(rename = "delete_message")]
    DeleteMessage { message_id: Uuid },

    #[serde(rename = "user_removed")]
    UserRemoved { chat: ChatDto, user: UserDto },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum OutgoingWsMessage {
    #[serde(rename = "new_message")]
    NewMessage { message: MessageDto },

    #[serde(rename = "message_edited")]
    MessageEdited { message: MessageDto },

    #[serde(rename = "message_deleted")]
    MessageDeleted { message: MessageDto },

    #[serde(rename = "user_added")]
    UserAdded { chat: ChatDto, user: UserDto },

    #[serde(rename = "user_removed")]
    UserRemoved { chat: ChatDto, user: UserDto },
}
