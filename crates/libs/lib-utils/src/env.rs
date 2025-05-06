use std::str::FromStr;

pub fn get_env(key: &str) -> Result<String> {
    let value = std::env::var(key).map_err(|_| Error::NotFound)?;

    if value.is_empty() {
        return Err(Error::Invalid);
    }

    Ok(value)
}

pub fn get_parsed_env<T: FromStr>(key: &str) -> Result<T> {
    let value = get_env(key)?;

    let value = value.parse::<T>().map_err(|_| Error::Invalid)?;

    Ok(value)
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Environment variable not found")]
    NotFound,

    #[error("Invalid environment variable value")]
    Invalid,
}
