pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing Git installation")]
    MissingGitInstallation,

    #[error("Invalid project kind {kind}")]
    InvalidProjectKind { kind: String },

    #[error("Failed to dermine rustx data directory")]
    MissingLocalDataDir,

    // --- Externals
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
