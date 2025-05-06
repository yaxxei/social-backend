pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Token generation error")]
    Generation,

    #[error("Token verification failed")]
    Verification,

    #[error("Token secret error")]
    Secret,

    #[error("Token expired")]
    Expired,

    #[error("Invalid token format")]
    InvalidFormat,
}
