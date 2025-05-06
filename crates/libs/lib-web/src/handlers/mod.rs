use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use lib_core::model::ModelManager;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

pub mod handlers_auth;
pub mod handlers_chat;
pub mod handlers_comment;
pub mod handlers_community;
pub mod handlers_follow;
pub mod handlers_like;
pub mod handlers_messages;
pub mod handlers_notification;
pub mod handlers_post;
pub mod handlers_profile;
pub mod handlers_user;
pub mod ws_handlers_chat;

#[derive(Debug, Clone)]
pub struct AppState {
    pub mm: Arc<ModelManager>,
    pub notification_conns: Arc<Mutex<HashMap<Uuid, SplitSink<WebSocket, Message>>>>,
    pub chat_conns: Arc<Mutex<HashMap<Uuid, Vec<UserConnection>>>>,
}

#[derive(Debug)]
pub struct UserConnection {
    pub user_id: Uuid,
    pub sender: mpsc::UnboundedSender<Message>,
}
