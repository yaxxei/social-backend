use chrono::NaiveDateTime;
use lib_core::db::Db;
use lib_core::model::save::{SaveForCreate, SaveForDelete, SaveForSelect, SaveRepo};
use serde::Serialize;
use uuid::Uuid;

use super::post_service::{PostDto, PostService};
use super::user_service::UserDto;

use crate::error::{Error, Result};
use crate::services::user_service::UserService;

#[derive(Serialize)]
pub struct SaveDto {
    id: Uuid,
    user_id: Uuid,
    post_id: Uuid,
    created_at: NaiveDateTime,
    user: UserDto,
    post: PostDto,
}

pub struct ProfileService;

impl ProfileService {
    pub async fn get_saves(db: &Db, requester_id: Option<Uuid>) -> Result<Vec<SaveDto>> {
        let user_id = match requester_id {
            Some(id) => id,
            None => return Err(Error::Unauthorized),
        };
        let save_fs = SaveForSelect {
            user_id: Some(user_id),
            ..Default::default()
        };

        let saves = SaveRepo::find_many(db, save_fs)
            .await?
            .into_iter()
            .map(|save| {
                let db = db.clone();
                async move {
                    let post = PostService::get_by_id(&db, requester_id, &save.post_id).await?;
                    let user = UserService::get_by_id(&db, requester_id, &user_id).await?;
                    Ok(SaveDto {
                        id: save.id,
                        user_id,
                        user,
                        post,
                        post_id: save.post_id,
                        created_at: save.created_at,
                    })
                }
            });

        futures::future::try_join_all(saves).await
    }

    pub async fn create_save(
        db: &Db,
        requester_id: Option<Uuid>,
        post_id: &Uuid,
    ) -> Result<SaveDto> {
        let save_fc = SaveForCreate {
            user_id: match requester_id {
                Some(id) => id,
                None => return Err(Error::Unauthorized),
            },
            post_id: *post_id,
        };

        let save = SaveRepo::create(db, save_fc).await?;
        let post = PostService::get_by_id(&db, requester_id, &save.post_id).await?;
        let user = UserService::get_by_id(&db, requester_id, &save.user_id).await?;
        Ok(SaveDto {
            id: save.id,
            user_id: save.user_id,
            post_id: save.post_id,
            created_at: save.created_at,
            user,
            post,
        })
    }

    pub async fn delete_save(db: &Db, requester_id: Option<Uuid>, post_id: &Uuid) -> Result<()> {
        let save_fd = SaveForDelete {
            user_id: match requester_id {
                Some(id) => id,
                None => return Err(Error::Unauthorized),
            },
            post_id: *post_id,
        };

        SaveRepo::delete(db, save_fd).await.map_err(|e| e.into())
    }

    pub async fn is_saved(db: &Db, requester_id: Option<Uuid>, post_id: &Uuid) -> Result<bool> {
        let save_fs = SaveForSelect {
            user_id: match requester_id {
                Some(id) => Some(id),
                None => return Ok(false),
            },
            post_id: Some(*post_id),
        };

        match SaveRepo::find(db, save_fs).await {
            Ok(_) => return Ok(true),
            Err(lib_core::error::Error::EntityNotFound) => return Ok(false),
            Err(e) => return Err(Error::Core(e)),
        }
    }
}
