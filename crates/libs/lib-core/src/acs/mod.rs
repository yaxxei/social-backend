// mod rbac (role-based access control)
use std::fmt::Display;

use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
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
        match (role, resource, action) {
            // Admin can everything
            (Role::Admin, _, _) => true,

            // Moderator can manage content
            (Role::Moderator, Resource::Post { .. }, Action::Delete) => true,
            (Role::Moderator, Resource::Comment { .. }, Action::Delete) => true,

            (Role::User, Resource::Post { .. }, Action::Create) => current_user_id.is_some(),
            (Role::User, Resource::Post { .. }, Action::Like | Action::Unlike) => {
                current_user_id.is_some()
            }
            (Role::User, Resource::Comment { .. }, Action::Create) => current_user_id.is_some(),
            (Role::User, Resource::Community { .. }, Action::Create) => current_user_id.is_some(),
            (Role::User, Resource::Community { .. }, Action::Follow | Action::Unfollow) => {
                current_user_id.is_some() && !Self::is_owner(resource, current_user_id)
            }

            // All can manage own content
            (_, _, Action::Update | Action::Delete) => Self::is_owner(resource, current_user_id),

            // All can read
            (_, _, Action::Read) => true,

            // Default deny
            _ => false,
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

// use std::fmt::Display;
// use uuid::Uuid;

// #[derive(Debug, Clone, Copy)]
// pub enum Action {
//     // Действия для постов
//     CreatePostInCommunity,
//     UpdatePost,
//     DeletePost,

//     // Действия для комментариев
//     CreateComment,
//     DeleteComment,

//     // Действия для сообществ
//     FollowCommunity,
//     JoinCommunity,
//     CreateCommunity,
//     UpdateCommunity,
//     DeleteCommunity,

//     // Действия для пользователей
//     UpdateUser,
//     DeleteUser,
// }

// impl Display for Action {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{self:?}")
//     }
// }

// #[derive(Debug, Clone)]
// pub enum Resource {
//     User(Uuid),
//     Community { id: Uuid, owner_id: Uuid, is_private: bool, members: Vec<Uuid> },
//     Post { id: Uuid, author_id: Uuid, community_id: Uuid },
//     Comment { id: Uuid, author_id: Uuid, post_id: Uuid },
// }

// pub struct AccessControl;

// impl AccessControl {
//     fn can(role: Role, resource: &Resource, action: Action, current_user_id: Option<Uuid>) -> bool {
//         match (role, resource, action) {
//             // Admin может всё
//             (Role::Admin, _, _) => true,

//             // Модератор может удалять посты и комментарии
//             (Role::Moderator, Resource::Post { .. }, Action::DeletePost) => true,
//             (Role::Moderator, Resource::Comment { .. }, Action::DeleteComment) => true,

//             // Пользователь может создавать посты и комментарии, если он участник сообщества
//             (Role::User, Resource::Post { community_id, .. }, Action::CreatePostInCommunity) => {
//                 current_user_id
//                     .map(|id| resource.is_member_of_community(id, *community_id))
//                     .unwrap_or(false)
//             }

//             // Создание комментариев
//             (Role::User, Resource::Comment { post_id, .. }, Action::CreateComment) => {
//                 current_user_id.is_some() // Должен быть залогинен
//             }

//             // Следить за сообществом
//             (Role::User, Resource::Community { id, is_private, members, .. }, Action::FollowCommunity) => {
//                 if *is_private {
//                     current_user_id
//                         .map(|id| members.contains(&id))
//                         .unwrap_or(false)
//                 } else {
//                     true
//                 }
//             }

//             // Присоединение к сообществу
//             (Role::User, Resource::Community { id, is_private, members, .. }, Action::JoinCommunity) => {
//                 if *is_private {
//                     current_user_id
//                         .map(|id| members.contains(&id))
//                         .unwrap_or(false)
//                 } else {
//                     true
//                 }
//             }

//             // Создание сообществ
//             (Role::User, Resource::Community { .. }, Action::CreateCommunity) => current_user_id.is_some(),

//             // Обновление сообществ
//             (_, Resource::Community { owner_id, .. }, Action::UpdateCommunity) => {
//                 Self::is_owner(resource, current_user_id)
//             }

//             // Удаление сообществ
//             (_, Resource::Community { owner_id, .. }, Action::DeleteCommunity) => {
//                 Self::is_owner(resource, current_user_id)
//             }

//             // Прочие действия, связанные с пользователем
//             (_, Resource::User(..), Action::UpdateUser) => true,
//             (_, Resource::User(..), Action::DeleteUser) => true,

//             // По умолчанию - запрет
//             _ => false,
//         }
//     }

//     fn is_owner(resource: &Resource, user_id: Option<Uuid>) -> bool {
//         match resource {
//             Resource::Post { author_id, .. } => user_id == Some(*author_id),
//             Resource::Comment { author_id, .. } => user_id == Some(*author_id),
//             Resource::User(resource_user_id) => user_id == Some(*resource_user_id),
//             Resource::Community { owner_id, .. } => user_id == Some(*owner_id),
//         }
//     }

//     pub fn check_access(
//         role: Role,
//         resource: Resource,
//         action: Action,
//         current_user_id: Option<Uuid>,
//     ) -> Result<(), Error> {
//         if Self::can(role.clone(), &resource, action, current_user_id) {
//             Ok(())
//         } else {
//             Err(Error::AccessDenied(AccessDenied {
//                 role,
//                 resource: resource.clone(),
//                 action,
//             }))
//         }
//     }
// }

// impl Resource {
//     fn is_member_of_community(&self, user_id: Uuid, community_id: Uuid) -> bool {
//         match self {
//             Resource::Community { id, members, .. } if *id == community_id => {
//                 members.contains(&user_id)
//             }
//             _ => false,
//         }
//     }
// }
