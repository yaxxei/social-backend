use chrono::NaiveDateTime;
use lib_core::acs::{AccessControl, Action, Resource, Role};
use lib_core::db::Db;
use lib_core::model::comment::{
    CommentForCreate, CommentForDelete, CommentForSelect, CommentForUpdate, CommentRepo,
};
use serde::Serialize;
use tracing::warn;
use uuid::Uuid;

use super::like_service::LikeService;
use super::post_service::PostService;
use super::user_service::UserService;

use crate::error::{Error, Result};
use crate::services::user_service::UserDto;

#[derive(Serialize, Clone)]
pub struct CommentDto {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub parent_comment_id: Option<Uuid>,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_deleted: bool,
    pub replies_count: u32,
    pub rating: i64,
    pub requester_like: Option<i16>,
    pub user: UserDto,
    pub post: CommentPost,
}

#[derive(Serialize, Clone)]
pub struct CommentPost {
    pub id: Uuid,
    pub title: String,
    pub community_name: String,
}

pub struct CommentService;

impl CommentService {
    pub async fn create(
        db: &Db,
        requester_id: Option<Uuid>,
        post_id: &Uuid,
        parent_comment_id: Option<Uuid>,
        content: &str,
    ) -> Result<CommentDto> {
        Self::check_access(
            &db,
            requester_id,
            Resource::Comment {
                id: Uuid::nil(),
                author_id: Uuid::nil(),
            },
            Action::Create,
        )
        .await?;

        let comment_fc = CommentForCreate {
            user_id: match requester_id {
                Some(id) => id,
                None => return Err(Error::Unauthorized),
            },
            post_id: *post_id,
            parent_comment_id,
            content: content.to_string(),
        };
        let comment = CommentRepo::create(db, comment_fc)
            .await
            .map_err(Error::Core)?;
        Self::convert_to_dto(db, requester_id, comment).await
    }

    pub async fn get_by_id(db: &Db, requester_id: Option<Uuid>, id: &Uuid) -> Result<CommentDto> {
        let comment_fs = CommentForSelect {
            id: Some(*id),
            ..Default::default()
        };
        let comment = CommentRepo::find(db, comment_fs)
            .await
            .map_err(Error::Core)?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Comment {
                id: comment.id,
                author_id: comment.user_id,
            },
            Action::Read,
        )
        .await?;

        Self::convert_to_dto(db, requester_id, comment).await
    }

    pub async fn get_many_by_user_id(
        db: &Db,
        requester_id: Option<Uuid>,
        user_id: &Uuid,
    ) -> Result<Vec<CommentDto>> {
        let user_id = UserService::get_by_id(db, requester_id, user_id)
            .await
            .map(|user| user.id)
            .ok();

        let comment_fs = CommentForSelect {
            user_id,
            ..Default::default()
        };
        let comments = CommentRepo::find_many(db, comment_fs)
            .await
            .map_err(Error::Core)?
            .into_iter()
            .map(|comment| {
                let db = db.clone();
                async move {
                    Self::check_access(
                        &db,
                        requester_id,
                        Resource::Comment {
                            id: comment.id,
                            author_id: comment.user_id,
                        },
                        Action::Read,
                    )
                    .await?;

                    Self::convert_to_dto(&db, requester_id, comment).await
                }
            });

        futures::future::try_join_all(comments).await
    }

    pub async fn get_many_by_post_id(
        db: &Db,
        requester_id: Option<Uuid>,
        post_id: &Uuid,
    ) -> Result<Vec<CommentDto>> {
        let comment_fs = CommentForSelect {
            post_id: Some(*post_id),
            ..Default::default()
        };
        let comments = CommentRepo::find_many(db, comment_fs)
            .await
            .map_err(Error::Core)?
            .into_iter()
            .map(|comment| {
                let db = db.clone();
                async move {
                    Self::check_access(
                        &db,
                        requester_id,
                        Resource::Comment {
                            id: comment.id,
                            author_id: comment.user_id,
                        },
                        Action::Read,
                    )
                    .await?;

                    Self::convert_to_dto(&db, requester_id, comment).await
                }
            });

        futures::future::try_join_all(comments).await
    }

    pub async fn get_comments_count(
        db: &Db,
        _requester_id: Option<Uuid>,
        post_id: &Uuid,
    ) -> Result<u32> {
        let comment_fs = CommentForSelect {
            post_id: Some(*post_id),
            ..Default::default()
        };
        CommentRepo::count(db, comment_fs)
            .await
            .map(|count| count as u32)
            .map_err(Error::Core)
    }

    pub async fn get_replies_count(
        db: &Db,
        requester_id: Option<Uuid>,
        comment_id: &Uuid,
    ) -> Result<u32> {
        let comment_fs = CommentForSelect {
            id: Some(*comment_id),
            ..Default::default()
        };
        let comment = CommentRepo::find(db, comment_fs)
            .await
            .map_err(Error::Core)?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Comment {
                id: comment.id,
                author_id: comment.user_id,
            },
            Action::Read,
        )
        .await?;

        let comment_fs = CommentForSelect {
            parent_comment_id: Some(*comment_id),
            ..Default::default()
        };
        CommentRepo::count(db, comment_fs)
            .await
            .map(|count| count as u32)
            .map_err(Error::Core)
    }

    pub async fn update(
        db: &Db,
        requester_id: Option<Uuid>,
        id: &Uuid,
        content: Option<String>,
    ) -> Result<CommentDto> {
        let comment_fs = CommentForSelect {
            id: Some(*id),
            ..Default::default()
        };
        let comment = CommentRepo::find(db, comment_fs)
            .await
            .map_err(Error::Core)?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Comment {
                id: comment.id,
                author_id: comment.user_id,
            },
            Action::Update,
        )
        .await?;

        let comment_fu = CommentForUpdate { content };
        let comment = CommentRepo::update(db, id, comment_fu)
            .await
            .map_err(Error::Core)?;
        Self::convert_to_dto(db, requester_id, comment).await
    }

    pub async fn delete(db: &Db, requester_id: Option<Uuid>, id: &Uuid) -> Result<()> {
        let comment_fs = CommentForSelect {
            id: Some(*id),
            ..Default::default()
        };
        let comment = CommentRepo::find(db, comment_fs)
            .await
            .map_err(Error::Core)?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Comment {
                id: comment.id,
                author_id: comment.user_id,
            },
            Action::Delete,
        )
        .await?;

        let comment_fd = CommentForDelete { id: *id };
        CommentRepo::delete(db, comment_fd).await?;

        Ok(())
    }

    async fn check_access(
        db: &Db,
        requester_id: Option<Uuid>,
        resource: Resource,
        action: Action,
    ) -> Result<()> {
        let role = match requester_id {
            Some(id) => get_role(db, &id).await?,
            None => Role::Guest,
        };

        AccessControl::check_access(role, resource, action, requester_id).map_err(|e| {
            warn!("{}", e.to_string());
            Error::Core(e.into())
        })
    }

    async fn convert_to_dto(
        db: &Db,
        requester_id: Option<Uuid>,
        comment: CommentRepo,
    ) -> Result<CommentDto> {
        let (replies_count, rating, requester_like, user, post) = tokio::try_join!(
            Self::get_replies_count(db, requester_id, &comment.id),
            LikeService::get_comment_rating(db, requester_id, &comment.id),
            LikeService::get_comment_like(db, requester_id, &comment.id),
            UserService::get_by_id(db, requester_id, &comment.user_id),
            PostService::get_by_id(db, requester_id, &comment.post_id)
        )?;

        Ok(CommentDto {
            id: comment.id,
            post_id: comment.post_id,
            user_id: comment.user_id,
            parent_comment_id: comment.parent_comment_id,
            content: comment.content,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            is_deleted: comment.is_deleted,
            replies_count,
            rating,
            requester_like,
            user,
            post: CommentPost {
                id: post.id,
                title: post.title,
                community_name: post.community.name,
            },
        })
    }
}

pub async fn get_role(db: &Db, user_id: &Uuid) -> Result<Role> {
    UserService::get_by_id(db, None, user_id)
        .await?
        .role
        .as_str()
        .parse()
        .map_err(|e| Error::Core(lib_core::error::Error::AccessControlSystem(e)))
}
