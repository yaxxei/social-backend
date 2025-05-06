pub mod error;

pub use self::error::{Error, Result};
pub use crate::config::auth_config;
use lib_utils::time::{utc_now_plus_days_usize, utc_now_plus_min_usize};

use std::str::FromStr;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

pub enum TokenType {
    Access,
    Refresh,
    ResetPassword,
}

pub struct Token {
    pub ident: String,
    pub exp: usize,
    pub sign: String,
}

impl Token {
    pub fn new(ident: &str, token_type: TokenType) -> Result<Self> {
        let exp = match token_type {
            TokenType::Access => utc_now_plus_min_usize(30),
            TokenType::Refresh => utc_now_plus_days_usize(30),
            TokenType::ResetPassword => utc_now_plus_min_usize(60),
        };

        let token = Self {
            ident: ident.to_string(),
            exp,
            sign: secret_by_type(token_type)?,
        };

        Ok(token)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: usize,
}

fn secret_by_type(token_type: TokenType) -> Result<String> {
    let secret: Result<&str> = match token_type {
        TokenType::Access => Ok(&auth_config().jwt_access_secret()),
        TokenType::Refresh => Ok(&auth_config().jwt_refresh_secret()),
        TokenType::ResetPassword => Ok(&auth_config().jwt_reset_password_secret()),
        _ => Err(Error::Secret),
    };
    let secret = secret?;

    Ok(secret.to_string())
}

pub fn generate_token(user: &str, token_type: TokenType) -> Result<String> {
    let token = Token::new(user, token_type)?;

    let token_claims = TokenClaims {
        sub: token.ident,
        exp: token.exp,
    };

    let token = encode(
        &Header::default(),
        &token_claims,
        &EncodingKey::from_secret(token.sign.as_ref()),
    )
    .map_err(|_| Error::Generation)?;

    Ok(token)
}

pub fn verify_token(token: &str, token_type: TokenType) -> Result<TokenData<TokenClaims>> {
    let secret = secret_by_type(token_type)?;

    let decoding_key = DecodingKey::from_secret(secret.as_ref());

    let mut validation = Validation::default();
    validation.validate_exp = true;

    let token_data = decode::<TokenClaims>(token, &decoding_key, &validation).map_err(|e| {
        if e.kind()
            .eq(&jsonwebtoken::errors::ErrorKind::ExpiredSignature)
        {
            Error::Expired
        } else {
            Error::Verification
        }
    })?;

    Ok(token_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};

    #[test]
    fn test_generate_and_verify_token() -> Result<()> {
        dotenvy::from_path(std::path::Path::new("../../.env")).ok();

        let exp = (chrono::Utc::now() + chrono::Duration::minutes(30)).timestamp() as usize;

        let access_token =
            generate_token("test_user", TokenType::Access).context("Failed to generate token")?;

        assert!(!access_token.is_empty(), "Access token should be generated");

        let decoded_data =
            verify_token(&access_token, TokenType::Access).context("Failed to verify token")?;

        assert_eq!(
            decoded_data.claims.sub, "test_user",
            "The 'sub' claim should match"
        );
        assert_eq!(decoded_data.claims.exp, exp, "The 'exp' claim should match");

        Ok(())
    }

    #[test]
    fn test_invalid_token_verification() -> Result<()> {
        dotenvy::from_path(std::path::Path::new("../../.env")).ok();

        let invalid_token = "invalid.token.string";

        let result = verify_token(invalid_token, TokenType::Access);

        assert!(
            result.is_err(),
            "It should return an error for invalid token"
        );

        Ok(())
    }
}
