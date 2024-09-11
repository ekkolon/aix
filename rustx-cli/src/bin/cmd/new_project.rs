// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{env::current_dir, path::PathBuf};

use clap::{ArgAction, Args, Parser, ValueEnum};
use rustx::{project::ProjectType, Context, TemplateEngine};
use serde::{Deserialize, Serialize};

use crate::{CONTEXT, TEMPLATE_ENGINE};

pub async fn run(args: &NewProjectArgs) -> rustx::Result<()> {
    if args.project_kind.standalone {
        scaffold_standalone(&CONTEXT, &TEMPLATE_ENGINE, args).await?;
    } else {
        scaffold_workspace(&CONTEXT, &TEMPLATE_ENGINE, args).await?;
    }

    Ok(())
}

async fn scaffold_standalone(
    ctx: &Context,
    template_engine: &TemplateEngine,
    args: &NewProjectArgs,
) -> rustx::Result<()> {
    let NewProjectArgs { name, root_dir, .. } = args;

    let out_dir = build_project_out_dir(name, root_dir)?;
    template_engine
        .write(ProjectType::Standalone, out_dir)
        .await?;

    Ok(())
}

async fn scaffold_workspace(
    ctx: &Context,
    template_engine: &TemplateEngine,
    args: &NewProjectArgs,
) -> rustx::Result<()> {
    let NewProjectArgs { name, root_dir, .. } = args;

    let out_dir = build_project_out_dir(name, root_dir)?;
    template_engine
        .write(ProjectType::Standalone, out_dir)
        .await?;

    Ok(())
}

fn build_project_out_dir(name: &str, root_dir: &Option<PathBuf>) -> rustx::Result<PathBuf> {
    match root_dir {
        Some(dir) => Ok(dir.join(name)),
        None => Ok(current_dir()?.join(name)),
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Parser)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields, default)]
pub struct NewProjectArgs {
    /// The project name.
    pub name: String,

    /// The directory at which to generate project files.
    ///
    /// Defaults to the project working directory.
    #[clap(value_name = "DIRECTORY")]
    pub root_dir: Option<PathBuf>,

    /// Whether to add a Dockerfile to the generated project.
    #[arg(long, action(ArgAction::SetTrue), default_value_t = false)]
    pub setup_docker: bool,

    /// Setup CI lint | test | build workflows
    #[arg(long)]
    pub ci: Option<ProjectCI>,

    #[clap(flatten)]
    #[serde(default = "ProjectKind::default")]
    project_kind: ProjectKind,
}

impl NewProjectArgs {
    pub fn project_type(&self) -> ProjectType {
        if self.project_kind.standalone {
            ProjectType::Standalone
        } else {
            ProjectType::Workspace
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Args)]
#[serde(rename_all = "camelCase")]
#[group(required = false, multiple = false)]
pub struct ProjectKind {
    /// Initializes a new standalone project (default)
    #[arg(
        long,
        action(ArgAction::SetTrue),
        default_value = "false",
        value_name = "TYPE",
        verbatim_doc_comment
    )]
    pub standalone: bool,

    /// Initializes a new workspace project
    #[arg(
        long,
        action(ArgAction::SetTrue),
        default_value = "false",
        value_name = "TYPE"
    )]
    #[serde(default)]
    pub workspace: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "camelCase")]
pub enum ProjectCI {
    #[default]
    Github,
}
