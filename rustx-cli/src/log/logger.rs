// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

use env_logger::Builder;
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init_logger(level: Option<LevelFilter>) {
    let level = level.unwrap_or(LevelFilter::Info);
    Builder::new()
        .format(|buf, record| {
            let (level_color, level_circle) = match record.level() {
                Level::Error => ("\x1b[31m", "●"), // Red
                Level::Warn => ("\x1b[33m", "●"),  // Yellow
                Level::Info => ("\x1b[32m", "●"),  // Green
                Level::Debug => ("\x1b[34m", "●"), // Blue
                Level::Trace => ("\x1b[35m", "●"), // Magenta
            };
            let reset = "\x1b[0m";

            let current_exe = std::env::current_exe()
                .unwrap_or_else(|_| "unknown".into())
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_uppercase();

            let current_exe = format!("\x1b[1m{current_exe}\x1b[0m");

            writeln!(
                buf,
                "{}{}{reset} {} {}{reset}",
                level_color,   // Color based on log level
                level_circle,  // Colored circle based on log level
                current_exe,   // Prepend BIN_NAME
                record.args(), // Log message
                reset = reset  // Reset ANSI codes
            )
        })
        .filter(None, level) // Enable all log levels
        .init();
}
