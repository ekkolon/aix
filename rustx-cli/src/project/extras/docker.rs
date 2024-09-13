// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{Config, Project};

/// Adds Docker configuration to a project.
///
/// # Arguments
///
/// - `app_handle`: An instance of `AppHandle` used for accessing application configuration and resources.
/// - `project`: The `Project` to which Docker configuration will be added.
///
/// # Returns
///
/// - `Ok(())` if the setup completes successfully.
/// - `Err` if an error occurs during setup, such as file copy failures.
pub async fn setup_docker(app_handle: &Config, project: &Project) -> crate::Result<()> {
    let docker_templates_dir = app_handle.templates_dir().join("extras").join("docker");

    crate::fs::copy_dir_all(docker_templates_dir, project.src_root()).await?;
    Ok(())
}
