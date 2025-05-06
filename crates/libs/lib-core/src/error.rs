use crate::acs;

pub type Result<T> = core::result::Result<T, Error>;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot manage with own entityt")]
    Own,

    #[error("Entity not found")]
    EntityNotFound,

    #[error("Entity `{entity}` is not unique: {unique}")]
    EntityNotUnique {
        entity: &'static str,
        unique: String,
    },

    #[error("Wrong password")]
    WrongPassword,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Failed to parse enum")]
    ParseEnumError,

    #[error("All fields are None")]
    AllNone,

    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),

    #[error(transparent)]
    SeaQuery(#[from] sea_query::error::Error),

    #[error(transparent)]
    Password(#[from] lib_auth::pwd::error::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error(transparent)]
    Redis(#[from] redis::RedisError),

    #[error(transparent)]
    AccessControlSystem(#[from] acs::Error),

    #[error("{0}")]
    InvalidInput(String),
}
