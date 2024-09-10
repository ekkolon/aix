// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::Parser;
use serde::{Deserialize, Serialize};

pub fn run(args: &NewArgs) -> rustx::Result<()> {
    println!("{args:#?}");
    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Parser)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields, default)]
pub struct NewArgs {
    /// The project name.
    pub name: String,

    #[arg(short = 't', long = "type", verbatim_doc_comment)]
    #[arg(value_name = "TYPE")]
    pub typ: Option<String>,
}
