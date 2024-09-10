pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // --- Externals
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
