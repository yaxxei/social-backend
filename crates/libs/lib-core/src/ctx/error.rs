pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Context cannot be new root")]
    CtxCannotNewRoot,
}
