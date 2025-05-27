pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Wrong password")]
    WrongPassword,

    #[error("Missing token in cookie")]
    MissingTokenCookie,

    #[error("Missing query params")]
    MissingQuery,

    #[error("User not found")]
    UserNotFound,

    #[error("Token not found")]
    TokenNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Unathorized")]
    Unauthorized,

    #[error("No required data passed")]
    NoRequiredDataPassed,

    #[error(transparent)]
    Ctx(#[from] lib_core::ctx::error::Error),

    #[error(transparent)]
    ToStrError(#[from] axum::http::header::ToStrError),

    #[error(transparent)]
    Uuid(#[from] uuid::Error),

    #[error(transparent)]
    Core(#[from] lib_core::error::Error),

    #[error(transparent)]
    Password(#[from] lib_auth::pwd::error::Error),

    #[error(transparent)]
    Token(#[from] lib_auth::token::error::Error),

    #[error(transparent)]
    JsonValidation(#[from] crate::extractors::JsonValidationError),

    #[error(transparent)]
    CtxExt(#[from] crate::extractors::CtxExtError),
}

impl Error {
    pub fn status_code(&self) -> u16 {
        match self {
            Error::Core(_) => 400,
            Error::Password(_) => 400,
            Error::Token(_) => 400,
            Error::WrongPassword => 401,
            Error::MissingTokenCookie => 401,
            Error::UserNotFound => 404,
            Error::Uuid(_) => 401,
            Error::UserAlreadyExists => 400,
            Error::MissingQuery => 400,
            Error::TokenNotFound => 404,
            Error::Unauthorized => 401,
            Error::ToStrError(_) => 400,
            Error::JsonValidation(_) => 422,
            Error::CtxExt(_) => 401,
            Error::Ctx(_) => 401,
            Error::NoRequiredDataPassed => 400, // ?
            Error::BadRequest(_) => 400,
        }
    }
}
