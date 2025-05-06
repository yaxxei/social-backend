use lib_core::db::Db;
use lib_core::model::like::{LikeForCreate, LikeForDelete, LikeForSelect, LikeRepo};
use uuid::Uuid;

use crate::error::{Error, Result};

pub struct LikeService;

impl LikeService {
    pub async fn like_post(
        db: &Db,
        requester_id: Option<Uuid>,
        post_id: &Uuid,
        like_type: i16,
    ) -> Result<LikeRepo> {
        let like_fc = LikeForCreate {
            user_id: match requester_id {
                Some(id) => id,
                None => return Err(Error::Unauthorized),
            },
            post_id: Some(*post_id),
            comment_id: None,
            like_type,
        };
        LikeRepo::create(db, like_fc).await.map_err(Error::Core)
    }

    pub async fn like_comment(
        db: &Db,
        requester_id: Option<Uuid>,
        comment_id: &Uuid,
        like_type: i16,
    ) -> Result<LikeRepo> {
        let like_fc = LikeForCreate {
            user_id: match requester_id {
                Some(id) => id,
                None => return Err(Error::Unauthorized),
            },
            post_id: None,
            comment_id: Some(*comment_id),
            like_type,
        };
        LikeRepo::create(db, like_fc).await.map_err(Error::Core)
    }

    pub async fn get_post_rating(
        db: &Db,
        _requester_id: Option<Uuid>,
        post_id: &Uuid,
    ) -> Result<i64> {
        LikeRepo::get_post_rating(db, post_id)
            .await
            .map_err(Error::Core)
    }

    pub async fn get_comment_rating(
        db: &Db,
        _requester_id: Option<Uuid>,
        comment_id: &Uuid,
    ) -> Result<i64> {
        LikeRepo::get_comment_rating(db, comment_id)
            .await
            .map_err(Error::Core)
    }

    // Аналогично можно добавить методы для работы с комментариями:
    pub async fn get_post_like(
        db: &Db,
        requester_id: Option<Uuid>,
        post_id: &Uuid,
    ) -> Result<Option<i16>> {
        let like_fs = LikeForSelect {
            user_id: match requester_id {
                Some(id) => Some(id),
                None => return Ok(None),
            },
            post_id: Some(*post_id),
            comment_id: None,
            ..Default::default()
        };
        Self::get_like(db, like_fs).await
    }

    pub async fn get_comment_like(
        db: &Db,
        requester_id: Option<Uuid>,
        comment_id: &Uuid,
    ) -> Result<Option<i16>> {
        let like_fs = LikeForSelect {
            user_id: match requester_id {
                Some(id) => Some(id),
                None => return Ok(None),
            },
            post_id: None,
            comment_id: Some(*comment_id),
            ..Default::default()
        };
        Self::get_like(db, like_fs).await
    }

    async fn get_like(db: &Db, like_fs: LikeForSelect) -> Result<Option<i16>> {
        match LikeRepo::find(db, like_fs).await {
            Ok(like) => Ok(Some(like.like_type)),
            Err(lib_core::error::Error::EntityNotFound) => Ok(None),
            Err(err) => Err(Error::Core(err)),
        }
    }

    pub async fn unlike_post(db: &Db, requester_id: Option<Uuid>, post_id: &Uuid) -> Result<()> {
        let like_fd = LikeForDelete {
            user_id: match requester_id {
                Some(id) => id,
                None => return Err(Error::Unauthorized),
            },
            post_id: Some(*post_id),
            comment_id: None,
        };
        LikeRepo::delete(db, like_fd).await.map_err(Error::Core)
    }

    pub async fn unlike_comment(
        db: &Db,
        requester_id: Option<Uuid>,
        comment_id: &Uuid,
    ) -> Result<()> {
        let like_fd = LikeForDelete {
            user_id: match requester_id {
                Some(id) => id,
                None => return Err(Error::Unauthorized),
            },
            comment_id: Some(*comment_id),
            post_id: None,
        };
        LikeRepo::delete(db, like_fd).await.map_err(Error::Core)
    }
}
