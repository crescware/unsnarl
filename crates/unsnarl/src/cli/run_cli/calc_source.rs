//! Resolve the source the pipeline runs against.
//!
//! Mirrors `ts/src/cli/run-cli/calc-source.ts`: feed stdin through
//! the `--stdin-lang` `SourceType` when `--stdin` is set, otherwise
//! require a positional file path. Returns a [`CliUsageError`]
//! (which the CLI converts to exit code 2) when neither input is
//! available — this is the same up-front guard the TS port runs
//! before any output-path resolution, so `uns -o build` with no
//! input cannot leak an empty `derived_basename` downstream.

use std::io::{self, Read};
use std::path::PathBuf;

use crate::cli::args::{Args, CliLanguage};
use crate::cli::run_cli::cli_usage_error::CliUsageError;

/// Outcome of [`calc_source`]: either stdin contents + the
/// declared stdin language, or a positional file path the
/// downstream `read_source` / language detector consumes.
#[derive(Debug)]
pub enum ExecuteSource {
    Stdin { text: String, lang: CliLanguage },
    File { path: PathBuf },
}

/// Resolve the source the pipeline runs against.
///
/// `stdin_reader` is parameterised on a `Read` impl so tests can
/// feed in-memory bytes without touching `io::stdin()`. The CLI
/// entry point passes `io::stdin()`; the integration tests pass
/// a `&[u8]` cursor.
pub fn calc_source<R: Read + ?Sized>(
    args: &Args,
    stdin_reader: &mut R,
    help: impl FnOnce() -> String,
) -> Result<ExecuteSource, CliUsageError> {
    if args.stdin {
        let mut text = String::new();
        stdin_reader
            .read_to_string(&mut text)
            .map_err(|e| CliUsageError::new(format!("failed to read stdin: {e}"), None))?;
        return Ok(ExecuteSource::Stdin {
            text,
            lang: args.stdin_lang.clone(),
        });
    }

    match &args.file {
        Some(path) => Ok(ExecuteSource::File { path: path.clone() }),
        None => Err(CliUsageError::new(
            "no input file (use --stdin or pass a path)",
            Some(help()),
        )),
    }
}

/// Convenience wrapper for the CLI binary that feeds `io::stdin()`
/// as the stdin source. The integration tests use [`calc_source`]
/// directly with an in-memory reader.
pub fn calc_source_from_stdin(
    args: &Args,
    help: impl FnOnce() -> String,
) -> Result<ExecuteSource, CliUsageError> {
    let mut stdin = io::stdin();
    calc_source(args, &mut stdin, help)
}

#[cfg(test)]
#[path = "calc_source_test.rs"]
mod calc_source_test;
