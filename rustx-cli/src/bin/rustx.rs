// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

mod cmd;

use clap::Parser;
use cmd::{Cli, Command};
use lazy_static::lazy_static;
use rustx::{Context, TemplateEngine};

lazy_static! {
    static ref CONTEXT: Context = Context::new("rustx").unwrap();
    static ref TEMPLATE_ENGINE: TemplateEngine = TemplateEngine {
        context: CONTEXT.clone(),
        git_origin: "git@github.com:ekkolon/rustx.git".into(),
        git_directory: "templates".into(),
        git_branch: "main".into(),
    };
}

#[tokio::main]
async fn main() -> rustx::Result<()> {
    env_logger::init();
    run().await?;
    Ok(())
}

async fn run() -> rustx::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::NewProject(args) => cmd::new_project::run(&args).await?,
    };

    Ok(())
}
