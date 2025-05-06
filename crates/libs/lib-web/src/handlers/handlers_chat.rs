use std::sync::Arc;

use axum::extract::{Path, State};
use serde::Serialize;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    extractors::CtxExt,
    services::chat_service::{ChatDto, ChatService},
    utils::response::ApiResponse,
};

use super::AppState;

pub async fn get_chats(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
) -> ApiResponse<ChatsResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch chats";
    info!("Starting fetch chats for user: {:?}", ctx.user_id);

    let chats = match ChatService::get_chats(state.mm.clone(), ctx.clone()).await {
        Ok(chats) => {
            info!("Chats fetched: {}", chats.len());
            chats
        }
        Err(err) => {
            error!("Failed to fetch chats for user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let chats_response = ChatsResponse { chats };

    info!("Chats fetched successully for user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Chats fetched successully", Some(chats_response))
}

pub async fn get_chat(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<ChatResponse> {
    const FAILED_MESSAGE: &str = "Failed to fetch chats";
    info!("Starting fetch chats for user: {:?}", ctx.user_id);

    let chat = match ChatService::get_chat(state.mm.clone(), ctx.clone(), &id).await {
        Ok(chat) => {
            info!("Chat fetched: {}", chat.id);
            chat
        }
        Err(err) => {
            error!("Failed to fetch chats for user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let chat_response = ChatResponse { chat };

    info!("Chat fetched successully for user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Chat fetched successully", Some(chat_response))
}

#[derive(Serialize)]
pub struct ChatsResponse {
    chats: Vec<ChatDto>,
}

#[derive(Serialize)]
pub struct ChatResponse {
    chat: ChatDto,
}
