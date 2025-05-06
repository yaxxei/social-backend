use std::str::FromStr;

use serde::Serialize;
use sqlx::Type;

use crate::error::Error;

#[derive(Debug, Serialize, Type, Clone, Copy, PartialEq)]
#[sqlx(type_name = "chat_role_enum")]
#[serde(rename_all = "snake_case")]
pub enum ChatRoleEnum {
    #[sqlx(rename = "owner")]
    Owner,

    #[sqlx(rename = "member")]
    Member,
}

impl ChatRoleEnum {
    pub fn as_str(&self) -> &str {
        match self {
            ChatRoleEnum::Owner => "owner",
            ChatRoleEnum::Member => "member",
        }
    }
}

impl FromStr for ChatRoleEnum {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "owner" => Ok(ChatRoleEnum::Owner),
            "member" => Ok(ChatRoleEnum::Member),
            _ => Err(Error::ParseEnumError),
        }
    }
}
