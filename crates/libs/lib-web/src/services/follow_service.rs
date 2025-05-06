use lib_core::acs::{AccessControl, Action, Resource, Role};
use lib_core::db::Db;
use lib_core::model::follow::{FollowForCreate, FollowForDelete, FollowForSelect, FollowRepo};
use tracing::warn;
use uuid::Uuid;

use super::community_service::CommunityService;
use super::user_service::UserService;

use crate::error::{Error, Result};

pub struct FollowService;

impl FollowService {
    pub async fn follow(
        db: &Db,
        requester_id: Option<Uuid>,
        community_id: &Uuid,
    ) -> Result<FollowRepo> {
        let community = CommunityService::get_by_id(db, requester_id, community_id).await?;
        Self::check_access(
            db,
            requester_id,
            Resource::Community {
                id: community.id,
                owner_id: community.user_id,
            },
            Action::Follow,
        )
        .await?;

        let follow_fc = FollowForCreate {
            user_id: requester_id.unwrap(),
            community_id: *community_id,
        };
        FollowRepo::create(db, follow_fc).await.map_err(Error::Core)
    }

    pub async fn get_followers(
        db: &Db,
        requester_id: Option<Uuid>,
        community_id: &Uuid,
    ) -> Result<Vec<FollowRepo>> {
        let community = CommunityService::get_by_id(db, requester_id, community_id).await?;
        Self::check_access(
            db,
            requester_id,
            Resource::Community {
                id: community.id,
                owner_id: community.user_id,
            },
            Action::Read,
        )
        .await?;

        let follow_fs = FollowForSelect {
            community_id: Some(*community_id),
            ..Default::default()
        };
        FollowRepo::find_many(db, follow_fs)
            .await
            .map_err(Error::Core)
    }

    pub async fn get_followings(
        db: &Db,
        requester_id: Option<Uuid>,
        user_id: &Uuid,
    ) -> Result<Vec<FollowRepo>> {
        let user = UserService::get_by_id(db, requester_id, user_id).await?;
        Self::check_access(db, requester_id, Resource::User(user.id), Action::Follow).await?;

        let follow_fs = FollowForSelect {
            user_id: Some(*user_id),
            ..Default::default()
        };
        FollowRepo::find_many(db, follow_fs)
            .await
            .map_err(Error::Core)
    }

    pub async fn get_followers_count(
        db: &Db,
        _requester_id: Option<Uuid>,
        community_id: &Uuid,
    ) -> Result<u32> {
        // let community = CommunityService::get_by_id(db, requester_id, community_id).await?;
        // Self::check_access(
        //     db,
        //     requester_id,
        //     Resource::Community {
        //         id: community.id,
        //         owner_id: community.user_id,
        //     },
        //     Action::Read,
        // )
        // .await?;

        let follow_fs = FollowForSelect {
            community_id: Some(*community_id),
            ..Default::default()
        };
        Ok(FollowRepo::count(db, follow_fs).await? as u32)
    }

    pub async fn is_followed(
        db: &Db,
        requester_id: Option<Uuid>,
        community_id: &Uuid,
    ) -> Result<bool> {
        let follow_fs = FollowForSelect {
            user_id: requester_id,
            community_id: Some(*community_id),
        };
        match FollowRepo::find(db, follow_fs).await {
            Ok(_) => Ok(true),
            Err(lib_core::error::Error::EntityNotFound) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn unfollow(db: &Db, requester_id: Option<Uuid>, community_id: &Uuid) -> Result<()> {
        let community = CommunityService::get_by_id(db, requester_id, community_id).await?;
        Self::check_access(
            db,
            requester_id,
            Resource::Community {
                id: community.id,
                owner_id: community.user_id,
            },
            Action::Unfollow,
        )
        .await?;

        let follow_fd = FollowForDelete {
            user_id: requester_id.unwrap(),
            community_id: *community_id,
        };
        FollowRepo::delete(db, follow_fd).await.map_err(Error::Core)
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
}

pub async fn get_role(db: &Db, user_id: &Uuid) -> Result<Role> {
    UserService::get_by_id(db, None, user_id)
        .await?
        .role
        .as_str()
        .parse()
        .map_err(|e| Error::Core(lib_core::error::Error::AccessControlSystem(e)))
}
