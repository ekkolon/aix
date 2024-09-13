// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::{Path, PathBuf};

use log::debug;
use tokio::fs;

use crate::{
    utils::git::{self, GitRepository},
    Error, Result,
};

pub const RELATIVE_TEMPLATES_DIR: &str = "templates";
pub const RELATIVE_TEMP_DIR: &str = ".tmp";

#[derive(Clone)]
pub struct Config {
    name: String,
    local_data_dir: PathBuf,
    templates_dir: PathBuf,
    temp_dir: PathBuf,
    git_repository: GitRepository,
}

impl Config {
    pub fn new(app_name: &str, git_repository: GitRepository) -> crate::Result<Self> {
        let app_local_data_dir = dirs::data_local_dir()
            .ok_or(crate::Error::MissingLocalDataDir)?
            .join(app_name);

        let app_templates_dir = app_local_data_dir.join(RELATIVE_TEMPLATES_DIR);
        let app_temp_dir = app_local_data_dir.join(RELATIVE_TEMP_DIR);

        Ok(Self {
            name: app_name.into(),
            local_data_dir: app_local_data_dir,
            templates_dir: app_templates_dir,
            temp_dir: app_temp_dir,
            git_repository,
        })
    }

    pub fn git_repository(&self) -> &GitRepository {
        &self.git_repository
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn local_data_dir(&self) -> &Path {
        &self.local_data_dir
    }

    pub fn templates_dir(&self) -> &Path {
        &self.templates_dir
    }

    pub fn temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    /// Pulls all project templates from the GitHub repository specified in self
    /// and copies them to the user's local data directory of the binary.
    pub async fn sync_store(&self) -> Result<()> {
        if !git::is_git_installed()? {
            return Err(Error::MissingGitInstallation);
        }

        let local_data_dir = self.local_data_dir();
        fs::create_dir_all(local_data_dir).await?;

        let temp_dir = self.temp_dir();
        fs::create_dir_all(temp_dir).await?;

        let git_repo = &self.git_repository();
        debug!("Cloning remote Git repository");
        git::clone(temp_dir, &git_repo.url, Some("."))?;

        debug!("Sparsed checkout init");
        git::sparse_checkout_init_cone(temp_dir)?;

        debug!("Set path for sparsed checkout");
        git::sparse_checkout_set_path(temp_dir, &git_repo.directory)?;

        // Copy template files
        let templates_dir = self.templates_dir();
        debug!(
            "Recreate template directories at {}",
            templates_dir.display()
        );
        crate::fs::recreate_dir(templates_dir).await?;

        debug!("Cleanup checkout repository before copy");
        crate::fs::remove_files_except(temp_dir, &["templates"]).await?;

        debug!("Copy fresh templates to local directory");
        let tmp_templates_dir = temp_dir.join(&git_repo.directory);
        crate::fs::copy_dir_all(tmp_templates_dir, templates_dir).await?;

        Ok(())
    }
}
