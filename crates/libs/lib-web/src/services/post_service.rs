use crate::services::user_service::UserDto;
use chrono::NaiveDateTime;
use lib_core::acs::{AccessControl, Action, Resource, Role};
use lib_core::db::Db;
use lib_core::model::post::{PostForCreate, PostForDelete, PostForSelect, PostForUpdate, PostRepo};
use serde::Serialize;
use tracing::warn;
use uuid::Uuid;

use super::comment_service::CommentService;
use super::community_service::{CommunityDto, CommunityService};
use super::like_service::LikeService;
use super::profile_service::ProfileService;
use super::user_service::UserService;

use crate::error::{Error, Result};

#[derive(Serialize)]
pub struct PostDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub community_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub comments_count: u32,
    pub rating: i64,
    pub requester_like: Option<i16>,
    pub is_saved: bool,
    pub user: UserDto,
    pub community: CommunityDto,
}

pub struct PostService;

impl PostService {
    pub async fn create(
        db: &Db,
        requester_id: Option<Uuid>,
        community_id: &Uuid,
        title: &str,
        content: &str,
    ) -> Result<PostDto> {
        Self::check_access(
            &db,
            requester_id,
            Resource::Post {
                id: Uuid::nil(),
                author_id: Uuid::nil(),
            },
            Action::Create,
        )
        .await?;

        let post_fc = PostForCreate {
            user_id: match requester_id {
                Some(id) => id,
                None => return Err(Error::Unauthorized),
            },
            community_id: *community_id,
            title: title.to_string(),
            content: content.to_string(),
        };
        let post = PostRepo::create(db, post_fc).await.map_err(Error::Core)?;
        Self::convert_to_dto(db, requester_id, post).await
    }

    pub async fn get_by_id(db: &Db, requester_id: Option<Uuid>, id: &Uuid) -> Result<PostDto> {
        let post_fs = PostForSelect {
            id: Some(*id),
            ..Default::default()
        };
        let post = PostRepo::find(db, post_fs).await.map_err(Error::Core)?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Post {
                id: post.id,
                author_id: post.user_id,
            },
            Action::Read,
        )
        .await?;

        Self::convert_to_dto(db, requester_id, post).await
    }

    pub async fn get_many(db: &Db, requester_id: Option<Uuid>) -> Result<Vec<PostDto>> {
        let post_fs = PostForSelect {
            ..Default::default()
        };
        let posts = PostRepo::find_many(db, post_fs)
            .await
            .map_err(Error::Core)?
            .into_iter()
            .map(|post| {
                let db = db.clone();
                async move {
                    Self::check_access(
                        &db,
                        requester_id,
                        Resource::Post {
                            id: post.id,
                            author_id: post.user_id,
                        },
                        Action::Read,
                    )
                    .await?;

                    Self::convert_to_dto(&db, requester_id, post).await
                }
            });

        futures::future::try_join_all(posts).await
    }

    pub async fn get_many_by_user_id(
        db: &Db,
        requester_id: Option<Uuid>,
        user_id: &Uuid,
    ) -> Result<Vec<PostDto>> {
        let user_id = UserService::get_by_id(db, requester_id, user_id)
            .await
            .map(|user| user.id)
            .ok();

        let post_fs = PostForSelect {
            user_id,
            ..Default::default()
        };
        let posts = PostRepo::find_many(db, post_fs)
            .await
            .map_err(Error::Core)?
            .into_iter()
            .map(|post| {
                let db = db.clone();
                async move {
                    Self::check_access(
                        &db,
                        requester_id,
                        Resource::Post {
                            id: post.id,
                            author_id: post.user_id,
                        },
                        Action::Read,
                    )
                    .await?;

                    Self::convert_to_dto(&db, requester_id, post).await
                }
            });

        futures::future::try_join_all(posts).await
    }

    pub async fn get_many_by_community_id(
        db: &Db,
        requester_id: Option<Uuid>,
        community_id: &Uuid,
    ) -> Result<Vec<PostDto>> {
        let community_id = CommunityService::get_by_id(db, requester_id, community_id)
            .await
            .map(|community| community.id)
            .ok();

        let post_fs = PostForSelect {
            community_id,
            ..Default::default()
        };
        let posts = PostRepo::find_many(db, post_fs)
            .await
            .map_err(Error::Core)?
            .into_iter()
            .map(|post| {
                let db = db.clone();
                async move {
                    Self::check_access(
                        &db,
                        requester_id,
                        Resource::Post {
                            id: post.id,
                            author_id: post.user_id,
                        },
                        Action::Read,
                    )
                    .await?;

                    Self::convert_to_dto(&db, requester_id, post).await
                }
            });

        futures::future::try_join_all(posts).await
    }

    pub async fn update(
        db: &Db,
        requester_id: Option<Uuid>,
        id: &Uuid,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<PostDto> {
        let post_fs = PostForSelect {
            id: Some(*id),
            ..Default::default()
        };
        let post = PostRepo::find(db, post_fs).await.map_err(Error::Core)?;

        Self::check_access(
            db,
            requester_id,
            Resource::Post {
                id: post.id,
                author_id: post.user_id,
            },
            Action::Update,
        )
        .await?;

        let post_fu = PostForUpdate { title, content };
        let post = PostRepo::update(db, id, post_fu)
            .await
            .map_err(Error::Core)?;
        Self::convert_to_dto(db, requester_id, post).await
    }

    pub async fn delete(db: &Db, requester_id: Option<Uuid>, id: &Uuid) -> Result<()> {
        let post_fs = PostForSelect {
            id: Some(*id),
            ..Default::default()
        };
        let post = PostRepo::find(db, post_fs).await.map_err(Error::Core)?;

        Self::check_access(
            db,
            requester_id,
            Resource::Post {
                id: post.id,
                author_id: post.user_id,
            },
            Action::Delete,
        )
        .await?;

        let post_fd = PostForDelete { id: *id };
        PostRepo::delete(db, post_fd).await.map_err(Error::Core)
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
        post: PostRepo,
    ) -> Result<PostDto> {
        Ok(PostDto {
            id: post.id,
            user_id: post.user_id,
            community_id: post.community_id,
            title: post.title,
            content: post.content,
            created_at: post.created_at,
            updated_at: post.updated_at,
            comments_count: CommentService::get_comments_count(db, requester_id, &post.id).await?,
            rating: LikeService::get_post_rating(db, requester_id, &post.id).await?,
            requester_like: LikeService::get_post_like(db, requester_id, &post.id).await?,
            is_saved: ProfileService::is_saved(db, requester_id, &post.id).await?,
            user: UserService::get_by_id(db, requester_id, &post.user_id).await?,
            community: CommunityService::get_by_id(db, requester_id, &post.community_id).await?,
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
