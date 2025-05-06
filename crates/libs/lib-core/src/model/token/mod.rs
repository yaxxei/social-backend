use crate::db::crud_fns::{create, delete, select, update};
use crate::db::{Db, DbEntity};
use crate::error::{Error, Result};
use derive_more::derive::Display;
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Display, Serialize, Deserialize, Type, Clone, Copy)]
#[sqlx(type_name = "token_type_enum")]
#[serde(rename_all = "snake_case")]
pub enum TokenTypeEnum {
    #[sqlx(rename = "access")]
    #[display("access")]
    Access,

    #[sqlx(rename = "refresh")]
    #[display("refresh")]
    Refresh,

    #[sqlx(rename = "reset_password")]
    #[display("reset_password")]
    ResetPassword,

    #[sqlx(rename = "reset_email")]
    #[display("reset_email")]
    ResetEmail,

    #[sqlx(rename = "email_verification")]
    #[display("email_verification")]
    EmailVerification,
}

impl FromStr for TokenTypeEnum {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "access" => Ok(TokenTypeEnum::Access),
            "refresh" => Ok(TokenTypeEnum::Refresh),
            "reset_password" => Ok(TokenTypeEnum::ResetPassword),
            "reset_email" => Ok(TokenTypeEnum::ResetEmail),
            "email_verification" => Ok(TokenTypeEnum::EmailVerification),
            _ => Err(Error::ParseEnumError),
        }
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Token {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub token_type: TokenTypeEnum,
}

#[derive(Serialize)]
pub struct TokenForCreate {
    pub user_id: Uuid,
    pub token: String,
    pub token_type: TokenTypeEnum,
}

#[derive(Serialize)]
pub struct TokenForSave {
    pub user_id: Uuid,
    pub token: String,
    pub token_type: TokenTypeEnum,
}

#[derive(Serialize)]
pub struct TokenForUpdate {
    pub token: String,
    pub token_type: TokenTypeEnum,
}

#[derive(Serialize, Default)]
pub struct TokenForSelect {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub token: Option<String>,
    pub token_type: Option<TokenTypeEnum>,
}

#[derive(Serialize)]
pub struct TokenForDelete {
    pub token: String,
}

impl DbEntity for Token {
    const TABLE: &'static str = "user_tokens";
}

impl Token {
    pub async fn create(db: &Db, token_fc: TokenForCreate) -> Result<Token> {
        create(db, token_fc).await
    }

    pub async fn update(db: &Db, id: &Uuid, token_fu: TokenForUpdate) -> Result<Token> {
        update(db, id, token_fu).await
    }

    pub async fn find(db: &Db, token_fs: TokenForSelect) -> Result<Token> {
        select(db, token_fs).await
    }

    pub async fn find_many(db: &Db, token_fs: TokenForSelect) -> Result<Token> {
        select(db, token_fs).await
    }

    pub async fn delete(db: &Db, token_fs: TokenForDelete) -> Result<()> {
        delete::<Token, _>(db, token_fs).await
    }
}
