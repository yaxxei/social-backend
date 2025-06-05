use std::{collections::HashSet, sync::Arc};

use chrono::NaiveDateTime;
use lib_core::{
    ctx::Ctx,
    model::{
        chat::{ChatForCreate, ChatForSelect, ChatForUpdate, ChatRepo},
        chat_member::{
            ChatMemberForCreate, ChatMemberForDelete, ChatMemberForSelect, ChatMemberRepo,
        },
        chat_role::ChatRoleEnum,
        message::{MessageForCreate, MessageForSelect, MessageForUpdate, MessageRepo},
        message_status::{MessageStatusForCreate, MessageStatusForSelect, MessageStatusRepo},
        ModelManager,
    },
};
use serde::{Deserialize, Serialize};
use tracing::warn;
use uuid::Uuid;

use crate::error::{Error, Result};

use super::user_service::{UserDto, UserService};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatDto {
    pub id: Uuid,
    pub name: String,
    pub is_group: bool,
    pub members_count: Option<u32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub unread_count: u32,
    pub last_message: Option<MessageDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageDto {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_edited: bool,
    pub is_deleted: bool,
    pub is_read: bool,
}

pub struct ChatService;

impl ChatService {
    pub async fn get_chats_meny_by_query(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        query: &str,
    ) -> Result<Vec<ChatDto>> {
        let chats_uid = ChatRepo::find_many_by_query(mm.db(), query)
            .await?
            .into_iter()
            .map(|chat| chat.id)
            .collect::<Vec<Uuid>>();

        let user_chats = Self::get_chats(mm.clone(), ctx).await?;

        let allowed_ids: HashSet<Uuid> = chats_uid.into_iter().collect();

        let filtered_chats: Vec<ChatDto> = user_chats
            .into_iter()
            .filter(|chat| allowed_ids.contains(&chat.id))
            .collect();

        Ok(filtered_chats)
    }

    pub async fn get_messages_meny_by_query(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        query: &str,
    ) -> Result<Vec<MessageDto>> {
        let msgs: Vec<MessageRepo> = MessageRepo::find_many_by_query(mm.db(), query).await?;

        let user_chats: Vec<ChatDto> = Self::get_chats(mm.clone(), ctx.clone()).await?;

        let chat_ids: HashSet<Uuid> = user_chats.iter().map(|chat| chat.id).collect();

        let filtered_msgs = msgs
            .into_iter()
            .filter(|msg| chat_ids.contains(&msg.chat_id))
            .map(|msg| {
                let mm = mm.clone();
                let ctx = ctx.clone();
                async move { Ok(Self::converte_message_to_dto(mm, ctx, msg).await?) }
            });

        futures::future::try_join_all(filtered_msgs).await
    }

    pub async fn get_chats(mm: Arc<ModelManager>, ctx: Ctx) -> Result<Vec<ChatDto>> {
        let db = mm.db();
        let user_id = ctx.user_id.ok_or(Error::Unauthorized)?;
        let chat_member_fs = ChatMemberForSelect {
            user_id: Some(user_id),
            ..Default::default()
        };

        let user_chat_members = ChatMemberRepo::find_all(db, chat_member_fs).await?;
        let chats = user_chat_members.into_iter().map(|cm| {
            let mm = mm.clone();
            let ctx = ctx.clone();

            async move {
                let chat = ChatRepo::find(
                    &db,
                    ChatForSelect {
                        id: Some(cm.chat_id),
                        ..Default::default()
                    },
                )
                .await?;

                Self::convert_chat_to_dto(mm, ctx, chat).await
            }
        });
        futures::future::try_join_all(chats).await
    }

    pub async fn get_chat(mm: Arc<ModelManager>, ctx: Ctx, chat_id: &Uuid) -> Result<ChatDto> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized)?;

        let chat = ChatRepo::find(
            mm.db(),
            ChatForSelect {
                id: Some(*chat_id),
                ..Default::default()
            },
        )
        .await?;

        Self::convert_chat_to_dto(mm, ctx, chat).await
    }

    pub async fn get_chat_owner(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        chat_id: &Uuid,
    ) -> Result<UserDto> {
        let chat_member = ChatMemberRepo::find(
            mm.db(),
            ChatMemberForSelect {
                chat_id: Some(*chat_id),
                role: Some(ChatRoleEnum::Owner),
                ..Default::default()
            },
        )
        .await?;
        let user = UserService::get_by_id(mm.db(), ctx.user_id, &chat_member.user_id).await?;
        Ok(user.into())
    }

    pub async fn get_unread_count(mm: Arc<ModelManager>, ctx: Ctx, chat_id: &Uuid) -> Result<u32> {
        let user_id = ctx.user_id.ok_or(Error::Unauthorized)?;

        let unread_count = MessageStatusRepo::find_all(
            mm.db(),
            MessageStatusForSelect {
                user_id: Some(user_id),
                chat_id: Some(*chat_id),
                is_read: Some(false),
                ..Default::default()
            },
        )
        .await?
        .len();

        Ok(unread_count as u32)
    }

    pub async fn get_messages(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        chat_id: &Uuid,
    ) -> Result<Vec<MessageDto>> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized)?;

        let messages = MessageRepo::find_all(
            mm.db(),
            MessageForSelect {
                chat_id: Some(*chat_id),
                ..Default::default()
            },
        )
        .await?
        .into_iter()
        .map(|msg| {
            let mm = mm.clone();
            let ctx = ctx.clone();
            async move { Self::converte_message_to_dto(mm, ctx, msg).await }
        });

        futures::future::try_join_all(messages).await
    }

    pub async fn get_message(mm: Arc<ModelManager>, ctx: Ctx, id: &Uuid) -> Result<MessageDto> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized)?;

        let message = MessageRepo::find(
            mm.db(),
            MessageForSelect {
                id: Some(*id),
                ..Default::default()
            },
        )
        .await?;

        Self::converte_message_to_dto(mm, ctx, message).await
    }

    pub async fn get_last_message(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        chat_id: &Uuid,
    ) -> Result<Option<MessageDto>> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized)?;

        let last_message = match MessageRepo::find_last_by_chat(mm.db(), chat_id).await {
            Ok(msg) => msg,
            Err(lib_core::error::Error::EntityNotFound) => return Ok(None),
            Err(e) => return Err(Error::Core(e)),
        };
        let last_message = match last_message {
            Some(msg) => msg,
            None => return Ok(None),
        };

        Ok(Some(
            Self::converte_message_to_dto(mm, ctx, last_message).await?,
        ))
    }

    pub async fn has_chat_with_user(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        user_id: &Uuid,
    ) -> Result<Option<ChatDto>> {
        let db = mm.db();
        let requester_id = ctx.user_id.ok_or(Error::Unauthorized)?;

        let requester_chat_members = ChatMemberRepo::find_all(
            db,
            ChatMemberForSelect {
                user_id: Some(requester_id),
                ..Default::default()
            },
        )
        .await?;
        let user_chat_members = ChatMemberRepo::find_all(
            db,
            ChatMemberForSelect {
                user_id: Some(*user_id),
                ..Default::default()
            },
        )
        .await?;

        let requester_chat_ids: HashSet<Uuid> =
            requester_chat_members.iter().map(|cm| cm.chat_id).collect();
        let user_chat_ids: HashSet<Uuid> = user_chat_members.iter().map(|cm| cm.chat_id).collect();

        let common_chat_ids: Vec<Uuid> = requester_chat_ids
            .intersection(&user_chat_ids)
            .cloned()
            .collect();

        for chat_id in common_chat_ids {
            let chat = ChatRepo::find(
                mm.db(),
                ChatForSelect {
                    id: Some(chat_id),
                    ..Default::default()
                },
            )
            .await?;

            if !chat.is_group {
                return Ok(Some(Self::convert_chat_to_dto(mm, ctx, chat).await?));
            }
        }

        Ok(None)
    }

    pub async fn create_chat(mm: Arc<ModelManager>, ctx: Ctx, name: &str) -> Result<ChatDto> {
        let requester_id = ctx.user_id.ok_or(Error::Unauthorized)?;

        let chat = ChatRepo::create(
            mm.db(),
            ChatForCreate {
                is_group: true,
                name: Some(name.to_string()),
            },
        )
        .await?;

        let _ = ChatMemberRepo::create(
            mm.db(),
            ChatMemberForCreate {
                chat_id: chat.id,
                user_id: requester_id,
                role: Some(ChatRoleEnum::Owner),
            },
        )
        .await?;

        Self::convert_chat_to_dto(mm, ctx, chat).await
    }

    pub async fn update_chat(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        id: &Uuid,
        name: &str,
    ) -> Result<ChatDto> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized)?;

        let chat = ChatRepo::update(
            mm.db(),
            id,
            ChatForUpdate {
                name: Some(name.to_string()),
                ..Default::default()
            },
        )
        .await?;

        Self::convert_chat_to_dto(mm, ctx, chat).await
    }

    pub async fn delete_chat(mm: Arc<ModelManager>, ctx: Ctx, id: &Uuid) -> Result<()> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized)?;

        let _ = ChatRepo::delete(mm.db(), id).await?;

        Ok(())
    }

    pub async fn add_user_to_group_chat(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        chat_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<(ChatDto, UserDto)> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized)?;

        let chat = ChatRepo::find(
            mm.db(),
            ChatForSelect {
                id: Some(*chat_id),
                ..Default::default()
            },
        )
        .await?;

        if !chat.is_group {
            return Err(Error::BadRequest(
                "You can't add users to a private chat".into(),
            ));
        }

        let existing_members = ChatMemberRepo::find_all(
            mm.db(),
            ChatMemberForSelect {
                chat_id: Some(*chat_id),
                user_id: Some(*user_id),
                ..Default::default()
            },
        )
        .await?;

        if !existing_members.is_empty() {
            return Err(Error::BadRequest("User already in the caht".into()));
        }

        let _ = ChatMemberRepo::create(
            mm.db(),
            ChatMemberForCreate {
                chat_id: *chat_id,
                user_id: *user_id,
                role: None,
            },
        )
        .await?;

        Ok((
            Self::convert_chat_to_dto(mm.clone(), ctx.clone(), chat).await?,
            UserService::get_by_id(mm.db(), ctx.user_id, user_id).await?,
        ))
    }

    pub async fn remove_user_from_group_chat(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        chat_id: &Uuid,
        user_id: &Uuid,
    ) -> Result<(ChatDto, UserDto)> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized)?;

        let chat = ChatRepo::find(
            mm.db(),
            ChatForSelect {
                id: Some(*chat_id),
                ..Default::default()
            },
        )
        .await?;

        if !chat.is_group {
            return Err(Error::BadRequest(
                "You can't remove users from a private chat".into(),
            ));
        }

        let existing_members = ChatMemberRepo::find(
            mm.db(),
            ChatMemberForSelect {
                chat_id: Some(*chat_id),
                user_id: Some(*user_id),
                ..Default::default()
            },
        )
        .await?;

        if existing_members.role == ChatRoleEnum::Owner {
            let _ = Self::delete_chat(mm.clone(), ctx.clone(), chat_id).await?;
        } else {
            let _ = ChatMemberRepo::delete(
                mm.db(),
                ChatMemberForDelete {
                    chat_id: Some(*chat_id),
                    user_id: Some(*user_id),
                },
            )
            .await?;
        }

        Ok((
            Self::convert_chat_to_dto(mm.clone(), ctx.clone(), chat).await?,
            UserService::get_by_id(mm.db(), ctx.user_id, user_id).await?,
        ))
    }

    pub async fn create_private_chat(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        user_id: &Uuid,
    ) -> Result<ChatDto> {
        let requester_id = ctx.user_id.ok_or(Error::Unauthorized)?;

        let _ = UserService::get_by_id(mm.db(), Some(requester_id), user_id)
            .await
            .map_err(|e| {
                warn!("User id does not exists in db");
                e
            })?;

        let chat = ChatRepo::create(
            mm.db(),
            ChatForCreate {
                is_group: false,
                name: None,
            },
        )
        .await?;

        let _ = ChatMemberRepo::create(
            mm.db(),
            ChatMemberForCreate {
                chat_id: chat.id,
                user_id: requester_id,
                role: None,
            },
        )
        .await?;

        let _ = ChatMemberRepo::create(
            mm.db(),
            ChatMemberForCreate {
                chat_id: chat.id,
                user_id: *user_id,
                role: None,
            },
        )
        .await?;

        Self::convert_chat_to_dto(mm, ctx, chat).await
    }

    pub async fn send_message(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        chat_id: &Uuid,
        content: &str,
    ) -> Result<MessageDto> {
        let requester_id = ctx.user_id.ok_or(Error::Unauthorized)?;
        let message = MessageRepo::create(
            mm.db(),
            MessageForCreate {
                chat_id: *chat_id,
                sender_id: requester_id,
                content: content.to_string(),
            },
        )
        .await?;

        let message_members = ChatMemberRepo::find_all(
            mm.db(),
            ChatMemberForSelect {
                chat_id: Some(message.chat_id),
                ..Default::default()
            },
        )
        .await?;

        for member in message_members {
            let is_sender = member.user_id == requester_id;

            let _ = MessageStatusRepo::create(
                mm.db(),
                MessageStatusForCreate {
                    message_id: message.id,
                    user_id: member.user_id,
                    chat_id: *chat_id,
                    is_send: true,
                    is_read: is_sender,
                    read_at: None,
                },
            )
            .await?;
        }

        Self::converte_message_to_dto(mm, ctx, message).await
    }

    pub async fn send_message_to_user(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        user_id: &Uuid,
        content: &str,
    ) -> Result<MessageDto> {
        let chat = Self::has_chat_with_user(mm.clone(), ctx.clone(), user_id).await?;

        if let Some(chat) = chat {
            let message = Self::send_message(mm, ctx, &chat.id, content).await?;
            Ok(message)
        } else {
            let chat = Self::create_private_chat(mm.clone(), ctx.clone(), user_id).await?;
            let message = Self::send_message(mm, ctx, &chat.id, content).await?;
            Ok(message)
        }
    }

    pub async fn update_message(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        id: &Uuid,
        content: &str,
    ) -> Result<MessageDto> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized);

        let message = MessageRepo::update(
            mm.db(),
            id,
            MessageForUpdate {
                content: Some(content.to_string()),
                ..Default::default()
            },
        )
        .await?;

        Self::converte_message_to_dto(mm, ctx, message).await
    }

    pub async fn read_message(mm: Arc<ModelManager>, ctx: Ctx, id: &Uuid) -> Result<()> {
        let user_id = ctx.user_id.ok_or(Error::Unauthorized)?;

        let _ = MessageStatusRepo::read_message(mm.db(), id, &user_id).await?;

        Ok(())
    }

    pub async fn delete_message(mm: Arc<ModelManager>, ctx: Ctx, id: &Uuid) -> Result<MessageDto> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized);

        let message = MessageRepo::delete(mm.db(), id).await?;
        Self::converte_message_to_dto(mm, ctx, message).await
    }

    pub async fn get_members(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        chat_id: &Uuid,
    ) -> Result<Vec<UserDto>> {
        let _ = ctx.user_id.ok_or(Error::Unauthorized);

        let chat_members = ChatMemberRepo::find_all(
            mm.db(),
            ChatMemberForSelect {
                chat_id: Some(*chat_id),
                ..Default::default()
            },
        )
        .await?;

        let users = chat_members.into_iter().map(|member| {
            let mm = mm.clone();
            let ctx = ctx.clone();
            async move { UserService::get_by_id(mm.db(), ctx.user_id, &member.user_id).await }
        });

        futures::future::try_join_all(users).await
    }

    pub async fn converte_message_to_dto(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        message: MessageRepo,
    ) -> Result<MessageDto> {
        let user_id = ctx.user_id.ok_or(Error::Unauthorized)?;
        let sender_name = UserService::get_by_id(mm.db(), ctx.user_id, &message.sender_id)
            .await?
            .nickname;

        let is_read = MessageStatusRepo::find(
            mm.db(),
            MessageStatusForSelect {
                message_id: Some(message.id),
                user_id: Some(user_id),
                ..Default::default()
            },
        )
        .await?
        .is_read;

        Ok(MessageDto {
            id: message.id,
            chat_id: message.chat_id,
            sender_id: message.sender_id,
            sender_name,
            content: message.content,
            created_at: message.created_at,
            updated_at: message.updated_at,
            is_edited: message.created_at != message.updated_at,
            is_deleted: message.is_deleted,
            is_read,
        })
    }

    pub async fn convert_chat_to_dto(
        mm: Arc<ModelManager>,
        ctx: Ctx,
        chat: ChatRepo,
    ) -> Result<ChatDto> {
        let reqeuster_id = ctx.user_id.ok_or(Error::Unauthorized)?;
        let unread_count: u32 = Self::get_unread_count(mm.clone(), ctx.clone(), &chat.id).await?;
        let last_message = Self::get_last_message(mm.clone(), ctx.clone(), &chat.id).await?;

        let name = match chat.is_group {
            true => chat.name.expect("Name not specified for chat"),
            false => {
                let user = Self::get_members(mm.clone(), ctx.clone(), &chat.id)
                    .await?
                    .into_iter()
                    .filter(|user| user.id.ne(&reqeuster_id))
                    .next()
                    .ok_or(Error::UserNotFound)?;

                let username = UserService::get_by_id(mm.db(), ctx.user_id, &user.id)
                    .await?
                    .nickname;

                username
            }
        };

        let members_count = Some(Self::get_members(mm, ctx, &chat.id).await?.len() as u32);

        Ok(ChatDto {
            id: chat.id,
            name,
            is_group: chat.is_group,
            members_count,
            created_at: chat.created_at,
            updated_at: chat.updated_at,
            unread_count,
            last_message,
        })
    }
}
