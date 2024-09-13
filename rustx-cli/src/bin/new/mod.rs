// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{env::current_dir, path::PathBuf};

use clap::{value_parser, ArgAction, Args, Parser};
use log::info;
use rustx::config::Config;
use rustx::project::extras::ci::CI;
use rustx::project::{Project, ProjectBuilder, ProjectType};
use serde::{Deserialize, Serialize};
use tokio::fs;

/// Creates and sets up a new project based on provided arguments.
///
/// # Arguments
///
/// - `app_handle`: An instance of `AppHandle` used to access application configuration and resources.
/// - `args`: An instance of `NewProjectArgs` containing the parameters for project creation and setup.
///
/// # Returns
///
/// - `Ok(())` if the project creation and setup are successful.
/// - `Err` if any error occurs during the process, such as file system errors or configuration issues.
///
/// # Description
///
/// This function orchestrates the creation of a new project by:
/// 1. Parsing the provided arguments to determine project type, name, and setup options.
/// 2. Building the project source directory based on the specified name and root directory.
/// 3. Using the `ProjectBuilder` to create and configure the project, including optional CI and Docker setups.
/// 4. Compiling the project files and printing a summary of the created files.
///
/// # Example
///
/// ```no_run
/// let args = NewProjectArgs {
///     name: "my_project".to_string(),
///     root_dir: None,
///     project_kind: ProjectKind { standalone: true, ..Default::default() },
///     setup_ci: Some(CI::Github),
///     setup_docker: true,
/// };
/// run(&app_handle, &args).await?;
/// ```
pub async fn run(app_handle: &Config, args: &NewProjectArgs) -> rustx::Result<()> {
    let NewProjectArgs {
        name,
        root_dir,
        project_kind,
        setup_ci,
        setup_docker,
    } = args;

    let project_type = ProjectType::from(project_kind);
    let project_src_root = build_project_out_dir(name, root_dir).await?;

    let project = ProjectBuilder::new(app_handle.clone())
        .typ(project_type)
        .name(name)
        .src_root(project_src_root)
        .setup_ci(setup_ci.is_some())
        .setup_docker(*setup_docker)
        .build()
        .await?;

    let project_files = project.compile().await?;
    print_new_project_files(&project, &project_files);

    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Parser)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields, default)]
pub struct NewProjectArgs {
    /// The name of the project.
    pub name: String,

    #[clap(flatten)]
    project_kind: ProjectKind,

    /// The directory where project files will be generated.
    /// Defaults to the current working directory if not specified.
    #[arg(
        short = 'd',
        long = "dir",
        value_name = "DIRECTORY",
        verbatim_doc_comment
    )]
    pub root_dir: Option<PathBuf>,

    /// Specifies a Continuous Integration provider to set up
    /// for the project (e.g., GitHub or GitLab).
    #[arg(long, value_name = "PROVIDER", value_parser = value_parser!(CI), verbatim_doc_comment)]
    pub setup_ci: Option<CI>,

    /// Indicates whether to add a Dockerfile to the generated project.
    /// Defaults to `false`
    ///
    #[arg(
        long,
        action(ArgAction::SetTrue),
        default_value_t = false,
        verbatim_doc_comment
    )]
    pub setup_docker: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Args)]
#[serde(rename_all = "camelCase")]
#[group(required = false, multiple = false)]
pub struct ProjectKind {
    /// Generates a new Rust project in standalone mode.
    /// This is the default mode.
    #[arg(
        long,
        action(ArgAction::SetTrue),
        default_value_t = false,
        verbatim_doc_comment
    )]
    pub standalone: bool,

    /// Generates a new Rust project in workspace mode.
    /// Creates a crate at the project root and adds it
    /// to the workspace's `Cargo.toml` file.
    #[arg(
        long,
        action(ArgAction::SetTrue),
        default_value_t = false,
        verbatim_doc_comment
    )]
    pub workspace: bool,
}

impl From<&ProjectKind> for ProjectType {
    fn from(kind: &ProjectKind) -> Self {
        if kind.standalone == kind.workspace | kind.standalone {
            ProjectType::Standalone
        } else {
            ProjectType::Workspace
        }
    }
}

fn print_new_project_files(project: &Project, file_list: &[String]) {
    let file_list_formatted = file_list
        .iter()
        .map(|path| format!("\x1b[38;2;46;111;64mADD\x1b[0m {path}"))
        .reduce(|prev: String, curr: String| format!("{prev}\n   {curr}"));

    if let Some(files) = file_list_formatted {
        info!(
            "Successfully created new {} project {}\n   {files}",
            project.typ().to_string(),
            project.name()
        );
    }
}

async fn build_project_out_dir(name: &str, root_dir: &Option<PathBuf>) -> rustx::Result<PathBuf> {
    let out_dir = match root_dir {
        Some(dir) => dir.to_path_buf(),
        None => current_dir()?.join(name),
    };

    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).await?;
    }

    if !out_dir.is_dir() {
        return Err(rustx::Error::NotADirectory { path: out_dir });
    }

    Ok(out_dir)
}
