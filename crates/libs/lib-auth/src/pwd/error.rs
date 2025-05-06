pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Passwird hashing error")]
    Hash,

    #[error("Password validation error")]
    Validate,
}
