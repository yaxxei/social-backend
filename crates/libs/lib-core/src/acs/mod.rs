// mod rbac (role-based access control)
use std::fmt::Display;

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
    Follow,
    Unfollow,
    Like,
    Unlike,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Admin,
    Moderator,
    User,
    Guest,
}

impl std::str::FromStr for Role {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "moderator" => Ok(Role::Moderator),
            "user" => Ok(Role::User),
            "guest" => Ok(Role::Guest),
            _ => Err(Error::UnknownRole(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Resource {
    User(Uuid),
    Community { id: Uuid, owner_id: Uuid },
    Post { id: Uuid, author_id: Uuid },
    Comment { id: Uuid, author_id: Uuid },
}

pub struct AccessControl;

impl AccessControl {
    fn can(role: Role, resource: &Resource, action: Action, current_user_id: Option<Uuid>) -> bool {
        match role {
            Role::Admin => {
                // Admin наследует все от Moderator и User
                true // либо вызови: Self::can(Role::Moderator, resource, action, current_user_id)
            }

            Role::Moderator => {
                // Moderator наследует от User
                match (resource, action) {
                    (Resource::Post { .. }, Action::Delete) => true,
                    (Resource::Comment { .. }, Action::Delete) => true,
                    _ => Self::can(Role::User, resource, action, current_user_id),
                }
            }

            Role::User => match (resource, action) {
                (Resource::Post { .. }, Action::Create) => current_user_id.is_some(),
                (Resource::Post { .. }, Action::Like | Action::Unlike) => current_user_id.is_some(),
                (Resource::Comment { .. }, Action::Create) => current_user_id.is_some(),
                (Resource::Community { .. }, Action::Create) => current_user_id.is_some(),
                (Resource::Community { .. }, Action::Follow | Action::Unfollow) => {
                    current_user_id.is_some() && !Self::is_owner(resource, current_user_id)
                }
                (_, Action::Update | Action::Delete) => Self::is_owner(resource, current_user_id),
                (_, Action::Read) => true,
                _ => false,
            },

            Role::Guest => match action {
                Action::Read => true,
                _ => false,
            },
        }
    }

    fn is_owner(resource: &Resource, user_id: Option<Uuid>) -> bool {
        match resource {
            Resource::Post { author_id, .. } => user_id == Some(*author_id),
            Resource::Comment { author_id, .. } => user_id == Some(*author_id),
            Resource::User(resource_user_id) => user_id == Some(*resource_user_id),
            Resource::Community { owner_id, .. } => user_id == Some(*owner_id),
        }
    }

    pub fn check_access(
        role: Role,
        resource: Resource,
        action: Action,
        current_user_id: Option<Uuid>,
    ) -> Result<(), Error> {
        if Self::can(role.clone(), &resource, action, current_user_id) {
            Ok(())
        } else {
            Err(Error::AccessDenied(AccessDenied {
                role,
                resource: resource.clone(),
                action,
            }))
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Access denied for {role:?} to {resource:?} for {action:?}")]
pub struct AccessDenied {
    pub role: Role,
    pub resource: Resource,
    pub action: Action,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unkown role: {0}")]
    UnknownRole(String),
    #[error(transparent)]
    AccessDenied(#[from] AccessDenied),
}
