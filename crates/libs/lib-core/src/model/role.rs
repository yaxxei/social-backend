use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::Type;

use crate::error::Error;

#[derive(Debug, Deserialize, Serialize, Type, Clone, Copy)]
#[sqlx(type_name = "role_enum")]
#[serde(rename_all = "snake_case")]
pub enum RoleEnum {
    #[sqlx(rename = "admin")]
    Admin,

    #[sqlx(rename = "moderator")]
    Moderator,

    #[sqlx(rename = "user")]
    User,
}

impl RoleEnum {
    pub fn as_str(&self) -> &str {
        match self {
            RoleEnum::Admin => "admin",
            RoleEnum::Moderator => "moderator",
            RoleEnum::User => "user",
        }
    }
}

impl FromStr for RoleEnum {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "admin" => Ok(RoleEnum::Admin),
            "moderator" => Ok(RoleEnum::Moderator),
            "user" => Ok(RoleEnum::User),
            _ => Err(Error::ParseEnumError),
        }
    }
}
