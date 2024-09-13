// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing Git installation")]
    MissingGitInstallation,

    #[error("Path {} is not a directory", path.display())]
    NotADirectory { path: PathBuf },

    #[error("Directory at {} is not empty", path.display())]
    DirectoryNotEmpty { path: PathBuf },

    #[error("Invalid project kind {kind}")]
    InvalidProjectKind { kind: String },

    #[error("Invalid project CI provider {provider}")]
    InvalidProjectCiProvider { provider: String },

    #[error("Failed to dermine rustx data directory")]
    MissingLocalDataDir,

    // --- Externals
    #[error(transparent)]
    Log(#[from] log::SetLoggerError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
