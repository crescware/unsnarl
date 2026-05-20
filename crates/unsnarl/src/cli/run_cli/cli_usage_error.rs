//! Marker error type for `uns` CLI usage failures.
//!
//! Mirrors `ts/src/cli/run-cli/cli-usage-error.ts`. The Rust port
//! routes through [`handle_cli_usage_error`](super::handle_cli_usage_error)
//! / `run.rs` to translate the error into an exit code 2 plus an
//! `error: <message>` line on stderr. `help` carries the
//! `command.helpInformation()` string when one was available at
//! throw-site; the orchestration prints it after the error line so
//! the user sees usage hints right alongside the diagnostic.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CliUsageError {
    pub message: String,
    pub help: Option<String>,
}

impl CliUsageError {
    pub fn new(message: impl Into<String>, help: Option<String>) -> Self {
        Self {
            message: message.into(),
            help,
        }
    }
}

impl fmt::Display for CliUsageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for CliUsageError {}
