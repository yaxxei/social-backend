use std::sync::Arc;

use axum::extract::{Path, Query, State};
use futures::SinkExt as _;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::Error,
    extractors::{CtxExt, ValidatedJson},
    handlers::ws_handlers_chat::OutgoingWsMessage,
    services::chat_service::{ChatService, MessageDto},
    utils::response::ApiResponse,
};

use super::AppState;

pub async fn get_message(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<MessageResposnse> {
    const FAILED_MESSAGE: &str = "Failed to fetch message";
    info!("Starting fetch message by user: {:?}", ctx.user_id);

    let message = match ChatService::get_message(state.mm.clone(), ctx.clone(), &id).await {
        Ok(msg) => {
            info!("Message fetched: {}", msg.id);
            msg
        }
        Err(err) => {
            error!("Failed to fetch message by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let post_response = MessageResposnse { message };

    info!("Message created successully by user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Message created successully", Some(post_response))
}

#[derive(Deserialize)]
pub struct MessageQuery {
    chat_id: Uuid,
}

pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<MessageQuery>,
) -> ApiResponse<MessagesResposnse> {
    const FAILED_MESSAGE: &str = "Failed to fetch messages";
    info!("Starting fetch messages by user: {:?}", ctx.user_id);

    let messages =
        match ChatService::get_messages(state.mm.clone(), ctx.clone(), &params.chat_id).await {
            Ok(msg) => {
                info!("Messages fetched: {}", msg.len());
                msg
            }
            Err(err) => {
                error!("Failed to fetch messages by user: {:?}", ctx.user_id);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };

    let msg_response = MessagesResposnse { messages };

    info!("Messages fetched successully by user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Messages fetched successully", Some(msg_response))
}

pub async fn create_message(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(payload): ValidatedJson<CreateMessagePayload>,
) -> ApiResponse<MessageResposnse> {
    const FAILED_MESSAGE: &str = "Failed to create message";
    info!("Starting create message by user: {:?}", ctx.user_id);

    let requester_id = match ctx.user_id.ok_or(Error::Unauthorized) {
        Ok(uid) => uid,
        Err(e) => {
            error!("Requester Unauthorized");
            return ApiResponse::error(FAILED_MESSAGE, e);
        }
    };

    if let Some(user_id) = payload.user_id {
        let message = match ChatService::send_message_to_user(
            state.mm.clone(),
            ctx.clone(),
            &user_id,
            &payload.content,
        )
        .await
        {
            Ok(msg) => {
                info!("Message created: {}", msg.id);
                if let Some(sender) = state.notification_conns.lock().await.get_mut(&user_id) {
                    let notif = OutgoingWsMessage::NewMessage {
                        message: msg.clone(),
                    };

                    if sender
                        .send(axum::extract::ws::Message::Text(
                            serde_json::to_string(&notif).unwrap().into(),
                        ))
                        .await
                        .is_ok()
                    {
                        debug!("new_message notification sent to user: {}", user_id);
                    } else {
                        warn!("Failed sent notification new_message to user: {}", user_id);
                    }
                } else {
                    warn!("Cannot get websocket connection to send notification");
                }
                msg
            }
            Err(err) => {
                error!("Failed to create message by user: {:?}", ctx.user_id);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };

        let post_response = MessageResposnse { message };

        info!("Message created successully by user: {:?}", ctx.user_id);
        return ApiResponse::success(201, "Message created successully", Some(post_response));
    }
    if let Some(chat_id) = payload.chat_id {
        let message = match ChatService::send_message(
            state.mm.clone(),
            ctx.clone(),
            &chat_id,
            &payload.content,
        )
        .await
        {
            Ok(msg) => {
                info!("Message created: {}", msg.id);
                match ChatService::get_members(state.mm.clone(), ctx.clone(), &msg.chat_id).await {
                    Ok(members) => {
                        for user_id in members
                            .into_iter()
                            .filter(|user| user.id.ne(&requester_id))
                            .map(|user| user.id)
                            .collect::<Vec<Uuid>>()
                        {
                            if let Some(sender) =
                                state.notification_conns.lock().await.get_mut(&user_id)
                            {
                                let notif = OutgoingWsMessage::NewMessage {
                                    message: msg.clone(),
                                };

                                if sender
                                    .send(axum::extract::ws::Message::Text(
                                        serde_json::to_string(&notif).unwrap().into(),
                                    ))
                                    .await
                                    .is_ok()
                                {
                                    debug!("new_message notification sent to user: {}", user_id);
                                } else {
                                    warn!(
                                        "Failed sent notification new_message to user: {}",
                                        user_id
                                    );
                                }
                            } else {
                                warn!("Cannot get websocket connection to send notification");
                            }
                        }
                    }
                    Err(_) => warn!("Cannot get chat members"),
                }
                msg
            }
            Err(err) => {
                error!("Failed to create message by user: {:?}", ctx.user_id);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };

        let post_response = MessageResposnse { message };

        info!("Message created successully by user: {:?}", ctx.user_id);
        return ApiResponse::success(201, "Message created successully", Some(post_response));
    } else {
        error!("Failed to create message by user: {:?}", ctx.user_id);
        return ApiResponse::error(FAILED_MESSAGE, Error::NoRequiredDataPassed);
    }
}

pub async fn update_message(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<UpdateMessagePayload>,
) -> ApiResponse<MessageResposnse> {
    const FAILED_MESSAGE: &str = "Failed to update message";
    info!("Starting update message by user: {:?}", ctx.user_id);

    let message =
        match ChatService::update_message(state.mm.clone(), ctx.clone(), &id, &payload.content)
            .await
        {
            Ok(msg) => {
                info!("Message updated: {}", msg.id);
                msg
            }
            Err(err) => {
                error!("Failed to update message by user: {:?}", ctx.user_id);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };

    let post_response = MessageResposnse { message };

    info!("Message updated successully by user: {:?}", ctx.user_id);
    ApiResponse::success(200, "Message updated successully", Some(post_response))
}

pub async fn delete_message(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<MessageResposnse> {
    const FAILED_MESSAGE: &str = "Failed to delete message";
    info!("Starting delete message by user: {:?}", ctx.user_id);

    let message = match ChatService::delete_message(state.mm.clone(), ctx.clone(), &id).await {
        Ok(msg) => {
            info!("Message deleted: {}", msg.id);
            msg
        }
        Err(err) => {
            error!("Failed to delete message by user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let post_response = MessageResposnse { message };

    info!("Message deleted successully by user: {:?}", ctx.user_id);
    ApiResponse::success(200, "Message deleted successully", Some(post_response))
}

pub async fn read_messages(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(payload): ValidatedJson<ReadMessagesPayload>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to read messages";
    info!(
        "Starting to mark messages as read by user: {:?}",
        ctx.user_id
    );

    for message_id in &payload.message_ids {
        if let Err(err) = ChatService::read_message(state.mm.clone(), ctx.clone(), message_id).await
        {
            error!(
                "Failed to read message {} by user {:?}: {}",
                message_id, ctx.user_id, err
            );
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    }

    info!(
        "Messages marked as read successfully by user: {:?}",
        ctx.user_id
    );
    ApiResponse::success(200, "Messages marked as read successfully", None)
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReadMessagesPayload {
    #[validate(length(min = 1, message = "At least one message ID is required"))]
    message_ids: Vec<Uuid>,
}

pub async fn read_message(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to read message";
    info!("Starting read message by user: {:?}", ctx.user_id);

    let _ = match ChatService::read_message(state.mm.clone(), ctx.clone(), &id).await {
        Ok(msg) => {
            info!("Message read: {}", id);
            msg
        }
        Err(err) => {
            error!("Failed to read message by user: {:?}: {}", ctx.user_id, err);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    info!("Message read successully by user: {:?}", ctx.user_id);
    ApiResponse::success(200, "Message read successully", None)
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMessagePayload {
    #[validate(length(min = 1, message = "Content is required"))]
    content: String,
    user_id: Option<Uuid>,
    chat_id: Option<Uuid>,
}
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMessagePayload {
    #[validate(length(min = 1, message = "Content is required"))]
    content: String,
}

#[derive(Serialize)]
pub struct MessageResposnse {
    message: MessageDto,
}

#[derive(Serialize)]
pub struct MessagesResposnse {
    messages: Vec<MessageDto>,
}
