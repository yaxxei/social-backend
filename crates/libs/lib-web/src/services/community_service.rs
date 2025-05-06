use chrono::NaiveDateTime;
use lib_core::acs::{AccessControl, Action, Resource, Role};
use lib_core::db::Db;
use lib_core::model::community::{
    CommunityForCreate, CommunityForDelete, CommunityForSelect, CommunityForUpdate, CommunityRepo,
};
use serde::Serialize;
use tracing::{instrument, warn};
use uuid::Uuid;

use super::follow_service::FollowService;
use super::user_service::UserService;

use crate::error::{Error, Result};
use crate::services::user_service::UserDto;

#[derive(Serialize)]
pub struct CommunityDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub is_private: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub followers_count: u32,
    pub is_followed: bool,
    pub user: UserDto,
}

pub struct CommunityService;

impl CommunityService {
    #[instrument(skip(db))]
    pub async fn create(
        db: &Db,
        requester_id: Option<Uuid>,
        name: &str,
        description: &str,
        is_private: &bool,
    ) -> Result<CommunityDto> {
        Self::check_access(
            &db,
            requester_id,
            Resource::Community {
                id: Uuid::nil(),
                owner_id: Uuid::nil(),
            },
            Action::Create,
        )
        .await?;

        let community_fc = CommunityForCreate {
            user_id: match requester_id {
                Some(id) => id,
                None => return Err(Error::Unauthorized),
            },
            name: name.to_string(),
            description: description.to_string(),
            is_private: *is_private,
        };
        let community = CommunityRepo::create(db, community_fc).await?;
        Self::convert_to_dto(db, community, requester_id).await
    }

    #[instrument(skip(db))]
    pub async fn get_by_id(db: &Db, requester_id: Option<Uuid>, id: &Uuid) -> Result<CommunityDto> {
        let community_fs = CommunityForSelect {
            id: Some(*id),
            ..Default::default()
        };
        let community = CommunityRepo::find(db, community_fs).await?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Community {
                id: community.id,
                owner_id: community.user_id,
            },
            Action::Read,
        )
        .await?;

        Self::convert_to_dto(db, community, requester_id).await
    }

    #[instrument(skip(db))]
    pub async fn get_by_name(
        db: &Db,
        requester_id: Option<Uuid>,
        name: &str,
    ) -> Result<CommunityDto> {
        let community_fs = CommunityForSelect {
            name: Some(name.to_string()),
            ..Default::default()
        };
        let community = CommunityRepo::find(db, community_fs).await?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Community {
                id: community.id,
                owner_id: community.user_id,
            },
            Action::Read,
        )
        .await?;

        Self::convert_to_dto(db, community, requester_id).await
    }

    #[instrument(skip(db))]
    pub async fn get_many(db: &Db, requester_id: Option<Uuid>) -> Result<Vec<CommunityDto>> {
        let community_fs = CommunityForSelect {
            ..Default::default()
        };
        let communities = CommunityRepo::find_all(db, community_fs)
            .await?
            .into_iter()
            .map(|community| {
                let db = db.clone();
                async move {
                    Self::check_access(
                        &db,
                        requester_id,
                        Resource::Community {
                            id: community.id,
                            owner_id: community.user_id,
                        },
                        Action::Read,
                    )
                    .await?;

                    Self::convert_to_dto(&db, community, requester_id).await
                }
            });

        futures::future::try_join_all(communities).await
    }

    #[instrument(skip(db))]
    pub async fn get_many_by_user_id(
        db: &Db,
        requester_id: Option<Uuid>,
        user_id: &Uuid,
    ) -> Result<Vec<CommunityDto>> {
        let community_fs = CommunityForSelect {
            user_id: Some(*user_id),
            ..Default::default()
        };
        let communities = CommunityRepo::find_all(db, community_fs)
            .await?
            .into_iter()
            .map(|community| {
                let db = db.clone();
                async move {
                    Self::check_access(
                        &db,
                        requester_id,
                        Resource::Community {
                            id: community.id,
                            owner_id: community.user_id,
                        },
                        Action::Read,
                    )
                    .await?;

                    Self::convert_to_dto(&db, community, requester_id).await
                }
            });

        futures::future::try_join_all(communities).await
    }

    #[instrument(skip(db))]
    pub async fn update(
        db: &Db,
        requester_id: Option<Uuid>,
        id: &Uuid,
        name: Option<String>,
        description: Option<String>,
        is_private: Option<bool>,
    ) -> Result<CommunityDto> {
        let community_fs = CommunityForSelect {
            id: Some(*id),
            ..Default::default()
        };
        let community = CommunityRepo::find(db, community_fs).await?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Community {
                id: community.id,
                owner_id: community.user_id,
            },
            Action::Update,
        )
        .await?;

        let community_fu = CommunityForUpdate {
            name,
            description,
            is_private,
        };
        let community = CommunityRepo::update(db, id, community_fu).await?;
        Self::convert_to_dto(db, community, requester_id).await
    }

    #[instrument(skip(db))]
    pub async fn delete(db: &Db, requester_id: Option<Uuid>, id: &Uuid) -> Result<()> {
        let community_fs = CommunityForSelect {
            id: Some(*id),
            ..Default::default()
        };
        let community = CommunityRepo::find(db, community_fs).await?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Community {
                id: community.id,
                owner_id: community.user_id,
            },
            Action::Delete,
        )
        .await?;

        let community_fd = CommunityForDelete { id: *id };
        CommunityRepo::delete(db, community_fd)
            .await
            .map_err(Error::Core)
    }

    #[instrument(skip(db))]
    pub async fn update_by_name(
        db: &Db,
        requester_id: Option<Uuid>,
        name_ident: &str,
        name: Option<String>,
        description: Option<String>,
        is_private: Option<bool>,
    ) -> Result<CommunityDto> {
        let community_fs = CommunityForSelect {
            name: Some(name_ident.to_string()),
            ..Default::default()
        };
        let community = CommunityRepo::find(db, community_fs).await?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Community {
                id: community.id,
                owner_id: community.user_id,
            },
            Action::Update,
        )
        .await?;

        let community_fu = CommunityForUpdate {
            name,
            description,
            is_private,
        };
        let community = CommunityRepo::update(db, &community.id, community_fu).await?;
        Self::convert_to_dto(db, community, requester_id).await
    }

    #[instrument(skip(db))]
    pub async fn delete_by_name(db: &Db, requester_id: Option<Uuid>, name: &str) -> Result<()> {
        let community_fs = CommunityForSelect {
            name: Some(name.to_string()),
            ..Default::default()
        };
        let community = CommunityRepo::find(db, community_fs).await?;

        Self::check_access(
            &db,
            requester_id,
            Resource::Community {
                id: community.id,
                owner_id: community.user_id,
            },
            Action::Delete,
        )
        .await?;

        let community_fd = CommunityForDelete { id: community.id };
        CommunityRepo::delete(db, community_fd)
            .await
            .map_err(Error::Core)
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
        community: CommunityRepo,
        requester_id: Option<Uuid>,
    ) -> Result<CommunityDto> {
        Ok(CommunityDto {
            id: community.id,
            user_id: community.user_id,
            name: community.name,
            description: community.description,
            is_private: community.is_private,
            created_at: community.created_at,
            updated_at: community.updated_at,
            followers_count: FollowService::get_followers_count(db, requester_id, &community.id)
                .await?,
            is_followed: FollowService::is_followed(db, requester_id, &community.id).await?,
            user: UserService::get_by_id(db, requester_id, &community.user_id).await?,
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
