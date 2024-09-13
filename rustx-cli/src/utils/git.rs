// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This module provides a set of functions for performing various Git operations
//! programmatically. It includes functionality for checking if Git is installed,
//! cloning repositories with sparse checkout settings, and configuring sparse
//! checkout in existing repositories.

use crate::Result;
use std::{
    path::Path,
    process::{Command, Stdio},
};

#[derive(Clone)]
pub struct GitRepository {
    pub url: String,
    pub branch: String,
    pub directory: String,
}

/// Checks if Git is installed on the system.
pub fn is_git_installed() -> Result<bool> {
    let child = Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    if child.is_err() {
        return Ok(false);
    }

    let installed = child.unwrap().wait().map(|_| true).unwrap_or(false);
    Ok(installed)
}

/// Clones a Git repository to a specified destination directory.
pub fn clone<P, D>(current_dir: P, origin: &str, destination: Option<D>) -> Result<()>
where
    P: AsRef<Path>,
    D: AsRef<str>,
{
    let mut child = Command::new("git");

    let mut cmd = child
        .current_dir(current_dir)
        .arg("clone")
        .arg("--filter=blob:none")
        .arg("--sparse")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg(origin);

    if let Some(dest) = destination {
        cmd = cmd.arg(dest.as_ref());
    }

    cmd.spawn()?.wait()?;
    Ok(())
}

/// Initializes sparse checkout in the current Git repository using the cone mode.
pub fn sparse_checkout_init_cone<P: AsRef<Path>>(current_dir: P) -> Result<()> {
    Command::new("git")
        .current_dir(current_dir)
        .arg("sparse-checkout")
        .arg("init")
        .arg("--cone")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;

    Ok(())
}

/// Configures the sparse checkout to include a specific path in the Git repository.
pub fn sparse_checkout_set_path<P: AsRef<Path>>(current_dir: P, path: &str) -> Result<()> {
    Command::new("git")
        .current_dir(current_dir)
        .arg("sparse-checkout")
        .arg("set")
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait()?;

    Ok(())
}
