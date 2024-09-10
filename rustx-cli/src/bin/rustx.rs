// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

mod cmd;

use clap::Parser;
use cmd::{Cli, Command};

fn main() -> rustx::Result<()> {
    run()
}

fn run() -> rustx::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::New(args) => {
            cmd::new::run(&args)?;
        }
    };

    Ok(())
}
