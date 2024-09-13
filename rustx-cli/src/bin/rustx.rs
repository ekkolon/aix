// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

mod new;
use new as new_project;

use log::LevelFilter;
use rustx::{Config, GitRepository};

use clap::{crate_authors, Args, Parser, Subcommand};

#[tokio::main]
async fn main() -> rustx::Result<()> {
    let cli = Cli::parse();

    // Set up env_logger with the chosen log level
    let log_level = get_log_level_filter(cli.global_args.verbose);
    std::env::set_var("RUST_LOG", log_level.to_string());
    rustx::logger::init_logger(Some(log_level));

    let handle = get_app_handle()?;

    match cli.command {
        Command::NewProject(args) => new::run(&handle, &args).await?,
    };

    Ok(())
}

#[derive(Parser)]
#[command(author = crate_authors!(), version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    pub global_args: GlobalArgs,
}

#[derive(Subcommand)]
pub enum Command {
    /// Generates a new Rust + Actix starter project.
    #[command(name = "new")]
    NewProject(new_project::NewProjectArgs),
}

#[derive(Args)]
pub struct GlobalArgs {
    /// Sets the level of verbosity (-v, -vv, -vvv)
    #[arg(short, long, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,
}

fn get_app_handle() -> rustx::Result<Config> {
    let app_name = std::env::current_exe()?;
    let app_name = app_name.file_name().unwrap().to_string_lossy();

    let git_repo = GitRepository {
        url: "git@github.com:ekkolon/rustx.git".into(),
        directory: "templates".into(),
        branch: "main".into(),
    };

    let app_handle = Config::new(&app_name, git_repo)?;
    Ok(app_handle)
}

fn get_log_level_filter(verbosity: u8) -> LevelFilter {
    // Map verbosity level to corresponding log level
    match verbosity {
        0 => LevelFilter::Info,  // Default level (no -v passed)
        1 => LevelFilter::Debug, // -v or more
        _ => LevelFilter::Trace, // -vv or more
    }
}
