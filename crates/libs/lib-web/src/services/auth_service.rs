use crate::{
    error::{Error, Result},
    services::user_service::UserService,
    utils::token::generate_tokens_for_auth,
};
use axum_extra::extract::CookieJar;
use lib_auth::{
    pwd::{hash_password, validate_password},
    token::{verify_token, TokenType},
};
use lib_core::model::token::{
    Token, TokenForCreate, TokenForDelete, TokenForSelect, TokenForUpdate, TokenTypeEnum,
};
use lib_core::model::ModelManager;
use std::{str::FromStr as _, sync::Arc};
use uuid::Uuid;

use crate::services::user_service::UserDto;
use crate::utils::cookies::{remove_cookie_from_jar, set_refresh_cookie};

pub struct AuthService;

impl AuthService {
    pub async fn register(
        mm: Arc<ModelManager>,
        jar: CookieJar,
        nickname: &str,
        email: &str,
        password: &str,
    ) -> Result<(UserDto, String, CookieJar)> {
        if UserService::get_by_nickname(mm.db(), None, nickname)
            .await
            .is_ok()
        {
            return Err(Error::UserAlreadyExists);
        }

        let hashed = hash_password(password)?;
        let user = UserService::create(mm.db(), None, nickname, email, &hashed).await?;

        Self::authenticate_user(mm, jar, user).await
    }

    pub async fn login(
        mm: Arc<ModelManager>,
        jar: CookieJar,
        nickname: &str,
        password: &str,
    ) -> Result<(UserDto, String, CookieJar)> {
        let user = UserService::get_by_nickname(mm.db(), None, nickname).await?;

        if !validate_password(password, &user.hashed_password)? {
            return Err(Error::WrongPassword);
        }

        Self::authenticate_user(mm, jar, user).await
    }

    pub async fn logout(mm: Arc<ModelManager>, jar: CookieJar) -> Result<CookieJar> {
        let token_cookie = jar.get("refreshToken").ok_or(Error::MissingTokenCookie)?;
        let token = token_cookie.value().to_string();

        let new_jar = remove_cookie_from_jar(jar, "refreshToken");
        Token::delete(mm.db(), TokenForDelete { token }).await?;
        Ok(new_jar)
    }

    pub async fn refresh(
        mm: Arc<ModelManager>,
        jar: CookieJar,
    ) -> Result<(UserDto, String, CookieJar)> {
        let token_cookie = jar.get("refreshToken").ok_or(Error::MissingTokenCookie)?;
        let token = token_cookie.value().to_string();

        let token_data = verify_token(&token, TokenType::Refresh).map_err(|e| {
            let _ = remove_cookie_from_jar(jar.clone(), "refreshToken");
            e
        })?;

        let user_id = Uuid::from_str(&token_data.claims.sub)?;
        let user = UserService::get_by_id(mm.db(), Some(user_id), &user_id).await?;

        Self::authenticate_user(mm, jar, user).await
    }

    async fn authenticate_user(
        mm: Arc<ModelManager>,
        jar: CookieJar,
        user: UserDto,
    ) -> Result<(UserDto, String, CookieJar)> {
        let (access_token, refresh_token) = generate_tokens_for_auth(&user)?;
        let new_jar = set_refresh_cookie(jar, &refresh_token);

        match Token::find(
            mm.db(),
            TokenForSelect {
                user_id: Some(user.id),
                ..Default::default()
            },
        )
        .await
        {
            Ok(token) => {
                let _ = Token::update(
                    mm.db(),
                    &token.id,
                    TokenForUpdate {
                        token: refresh_token,
                        token_type: TokenTypeEnum::Refresh,
                    },
                )
                .await;
            }
            Err(_) => {
                let _ = Token::create(
                    mm.db(),
                    TokenForCreate {
                        user_id: user.id,
                        token: refresh_token,
                        token_type: TokenTypeEnum::Refresh,
                    },
                )
                .await?;
            }
        };
        Ok((user, access_token, new_jar))
    }
}
