// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use crate::{Config, Project};

/// A Continuous Integration (CI) provider.
///
/// This enum represents different CI providers such as GitHub or GitLab.
///
/// # Variants
///
/// - `Github`: Using GitHub Actions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CI {
    #[default]
    GitHub,
}

impl Display for CI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &CI::GitHub => write!(f, "github"),
        }
    }
}

impl FromStr for CI {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "github" => Ok(Self::GitHub),
            _ => Err(crate::Error::InvalidProjectCiProvider { provider: s.into() }),
        }
    }
}

/// Adds CI configuration to a project based on the specified CI provider.
///
/// # Arguments
///
/// - `app_handle`: An instance of `AppHandle` used for accessing application configuration and resources.
/// - `project`: The `Project` to which the CI configuration will be added.
/// - `ci`: The CI provider to configure (e.g., `CI::Github`).
///
/// # Returns
///
/// - `Ok(())` if the setup completes successfully.
/// - `Err` if an error occurs during setup, such as file copy failures.
pub async fn setup_ci(app_handle: &Config, project: &Project, ci: &CI) -> crate::Result<()> {
    let docker_templates_dir = app_handle
        .templates_dir()
        .join("extras")
        .join("ci")
        .join(ci.to_string());

    match *ci {
        CI::GitHub => {
            let github_ci_template_dir = docker_templates_dir.join(".github");
            let github_ci_project_dir = project.src_root().join(".github");
            crate::fs::copy_dir_all(github_ci_template_dir, github_ci_project_dir).await?;
        }
    }

    Ok(())
}
