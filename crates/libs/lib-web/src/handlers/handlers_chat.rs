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
    services::{
        chat_service::{ChatDto, ChatService, MessageDto},
        user_service::{UserDto, UserService},
    },
    utils::response::ApiResponse,
};

use super::AppState;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

pub async fn search(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<SearchQuery>,
) -> ApiResponse<SearchResponse> {
    const FAILED_MESSAGE: &str = "Failed to search";
    info!("Starting search for user: {:?}", ctx.user_id);

    let users =
        match UserService::get_meny_by_query(state.mm.db(), ctx.user_id, &params.query).await {
            Ok(users) => users,
            Err(e) => {
                error!("Error while searching users");
                return ApiResponse::error(FAILED_MESSAGE, e);
            }
        };

    let chats =
        match ChatService::get_chats_meny_by_query(state.mm.clone(), ctx.clone(), &params.query)
            .await
        {
            Ok(communities) => communities,
            Err(e) => {
                error!("Error while searching chats");
                return ApiResponse::error(FAILED_MESSAGE, e);
            }
        };

    let messages =
        match ChatService::get_messages_meny_by_query(state.mm.clone(), ctx.clone(), &params.query)
            .await
        {
            Ok(posts) => posts,
            Err(e) => {
                error!("Error while searching messages");
                return ApiResponse::error(FAILED_MESSAGE, e);
            }
        };

    let search_response = SearchResponse {
        users,
        chats,
        messages,
    };

    info!("Searched successully for user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Searched successully", Some(search_response))
}

#[derive(Deserialize)]
pub struct HasChatQuery {
    user_id: Uuid,
}

pub async fn has_chats_with_user(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Query(params): Query<HasChatQuery>,
) -> ApiResponse<ChatResponse> {
    const FAILED_MESSAGE: &str = "Failed to check chat";
    info!("Starting check chat for user: {:?}", ctx.user_id);

    let chat = match ChatService::has_chat_with_user(state.mm.clone(), ctx.clone(), &params.user_id)
        .await
    {
        Ok(chats) => match chats {
            Some(chat) => {
                info!("Chat found");
                chat
            }
            None => {
                warn!("Chat not found with user: {}", params.user_id);
                return ApiResponse::error(FAILED_MESSAGE, Error::UserNotFound);
            }
        },
        Err(err) => {
            error!("Failed to check chats for user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let chat_response = ChatResponse { chat };

    info!("Chats found successully for user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Chats found successully", Some(chat_response))
}

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

pub async fn create_chat(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    ValidatedJson(payload): ValidatedJson<CreateChatPayload>,
) -> ApiResponse<ChatResponse> {
    const FAILED_MESSAGE: &str = "Failed to create chat";
    info!("Starting create chat for user: {:?}", ctx.user_id);

    let chat = match ChatService::create_chat(state.mm.clone(), ctx.clone(), &payload.name).await {
        Ok(chat) => {
            info!("Chat created: {}", chat.id);
            chat
        }
        Err(err) => {
            error!("Failed to create chat for user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let chat_response = ChatResponse { chat };

    info!("Chat created successully for user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Chat created successully", Some(chat_response))
}

pub async fn update_chat(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<CreateChatPayload>,
) -> ApiResponse<ChatResponse> {
    const FAILED_MESSAGE: &str = "Failed to update chat";
    info!("Starting update chat for user: {:?}", ctx.user_id);

    let chat =
        match ChatService::update_chat(state.mm.clone(), ctx.clone(), &id, &payload.name).await {
            Ok(chat) => {
                info!("Chat updated: {}", chat.id);
                chat
            }
            Err(err) => {
                error!("Failed to update chat for user: {:?}", ctx.user_id);
                return ApiResponse::error(FAILED_MESSAGE, err);
            }
        };

    let chat_response = ChatResponse { chat };

    info!("Chat updated successully for user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Chat updated successully", Some(chat_response))
}

pub async fn delete_chat(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(id): Path<Uuid>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to delete chat";
    info!("Starting delete chat for user: {:?}", ctx.user_id);

    let _ = match ChatService::delete_chat(state.mm.clone(), ctx.clone(), &id).await {
        Ok(_) => {
            info!("Chat deleted: {}", id);
        }
        Err(err) => {
            error!("Failed to delete chat for user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    info!("Chat deleted successully for user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Chat deleted successully", None)
}

pub async fn add_user_to_group_chat(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(chat_id): Path<Uuid>,
    ValidatedJson(payload): ValidatedJson<AddUserToGroupChatPayload>,
) -> ApiResponse<()> {
    const FAILED_MESSAGE: &str = "Failed to add user to group chat";
    info!(
        "Adding user {:?} to group chat {:?} by user {:?}",
        payload.user_id, chat_id, ctx.user_id
    );

    let requester_id = match ctx.user_id.ok_or(Error::Unauthorized) {
        Ok(uid) => uid,
        Err(e) => {
            error!("Requester Unauthorized");
            return ApiResponse::error(FAILED_MESSAGE, e);
        }
    };

    let result = ChatService::add_user_to_group_chat(
        state.mm.clone(),
        ctx.clone(),
        &chat_id,
        &payload.user_id,
    )
    .await;

    match result {
        Ok(nickname) => {
            info!(
                "User {:?} added to group chat {:?} successfully",
                payload.user_id, chat_id
            );

            match ChatService::get_members(state.mm.clone(), ctx.clone(), &chat_id).await {
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
                            let notif = OutgoingWsMessage::UserAdded {
                                nickname: nickname.clone(),
                            };

                            if sender
                                .send(axum::extract::ws::Message::Text(
                                    serde_json::to_string(&notif).unwrap().into(),
                                ))
                                .await
                                .is_ok()
                            {
                                debug!("message_updated notification sent to user: {}", user_id);
                            } else {
                                warn!(
                                    "Failed sent notification message_updated to user: {}",
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

            ApiResponse::success(200, "User added to group chat", None)
        }
        Err(err) => {
            error!("Failed to add user to group chat: {:?}", err);
            ApiResponse::error(FAILED_MESSAGE, err)
        }
    }
}

pub async fn get_members(
    State(state): State<Arc<AppState>>,
    CtxExt(ctx): CtxExt,
    Path(chat_id): Path<Uuid>,
) -> ApiResponse<MembersResponse> {
    const FAILED_MESSAGE: &str = "Failed to create chat";
    info!("Starting create chat for user: {:?}", ctx.user_id);

    let members = match ChatService::get_members(state.mm.clone(), ctx.clone(), &chat_id).await {
        Ok(members) => {
            info!("Members fetched: {}", members.len());
            members
        }
        Err(err) => {
            error!("Failed to fetch members for user: {:?}", ctx.user_id);
            return ApiResponse::error(FAILED_MESSAGE, err);
        }
    };

    let members_response = MembersResponse { members };

    info!("Chat created successully for user: {:?}", ctx.user_id);
    ApiResponse::success(201, "Chat created successully", Some(members_response))
}

#[derive(Deserialize, Validate)]
pub struct CreateChatPayload {
    #[validate(length(min = 1, message = "Name is required"))]
    name: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateChatPayload {
    #[validate(length(min = 1, message = "Name is required"))]
    name: String,
}

#[derive(Deserialize, Validate)]
pub struct AddUserToGroupChatPayload {
    user_id: Uuid,
}

#[derive(Serialize)]
pub struct ChatsResponse {
    chats: Vec<ChatDto>,
}

#[derive(Serialize)]
pub struct ChatResponse {
    chat: ChatDto,
}

#[derive(Serialize)]
pub struct MembersResponse {
    members: Vec<UserDto>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    users: Vec<UserDto>,
    chats: Vec<ChatDto>,
    messages: Vec<MessageDto>,
}
