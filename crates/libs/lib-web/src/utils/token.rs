use crate::services::user_service::UserDto;
use lib_auth::token::{generate_token, TokenType};

use crate::error::Result;

pub(crate) fn generate_tokens_for_auth(user: &UserDto) -> Result<(String, String)> {
    let access_token = generate_token(&user.id.to_string(), TokenType::Access)?;
    let refresh_token = generate_token(&user.id.to_string(), TokenType::Refresh)?;

    Ok((access_token, refresh_token))
}
