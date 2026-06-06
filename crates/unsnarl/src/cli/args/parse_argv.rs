//! Hand-rolled argv parser for the `uns` CLI.
//!
//! Replaces the `clap` derive that used to drive [`super::Args`].
//! The accepted-input set is preserved byte-for-byte against the
//! previous clap surface; only the wording of error messages may
//! differ (see `parse_error::ParseError`).
//!
//! Supported flag forms, mirroring clap defaults:
//!
//! - Long: `--name`, `--name value`, `--name=value`
//! - Short value-taking: `-X value`, `-Xvalue`, `-X=value`
//! - Boolean short: `-X` only (no bundling — none of unsnarl's
//!   boolean flags use short forms, so `-vh` style bundling is
//!   not implemented)
//! - `--` terminator: every following token is treated as a
//!   positional argument
//!
//! Value-taking flags do NOT consume the next token if it starts with
//! `-` (and is not exactly `-`). This matches clap's default
//! `allow_hyphen_values = false`. Cases like `-A -1` therefore yield
//! a "missing value" failure rather than feeding `-1` to the integer
//! parser; either way the exit code is 2, so the existing tests are
//! unaffected.

use std::ffi::OsString;
use std::path::PathBuf;

use unsnarl_root_query::{parse_highlight_queries, parse_root_queries};

use super::help_text::{version_text, HELP_TEXT};
use super::highlight::Highlight;
use super::parse_error::ParseError;
use super::Args;

mod handle_highlight;
mod handle_long;
mod handle_short;
mod values;

use handle_long::handle_long;
use handle_short::handle_short;

pub fn parse_argv<I, T>(argv: I) -> Result<Args, ParseError>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
{
    let argv: Vec<OsString> = argv.into_iter().map(Into::into).collect();
    let mut args = Args::default();
    let mut i = 1; // skip argv[0]
    let mut after_terminator = false;
    let mut positional_count = 0usize;
    while i < argv.len() {
        let raw = &argv[i];
        if after_terminator {
            apply_positional(&mut args, raw, &mut positional_count)?;
            i += 1;
            continue;
        }
        let Some(s) = raw.to_str() else {
            apply_positional(&mut args, raw, &mut positional_count)?;
            i += 1;
            continue;
        };
        if s == "--" {
            after_terminator = true;
            i += 1;
            continue;
        }
        if s == "--help" || s == "-h" {
            return Err(ParseError::display_help(HELP_TEXT));
        }
        if s == "--version" || s == "-v" {
            return Err(ParseError::display_version(version_text()));
        }
        if let Some(long) = s.strip_prefix("--") {
            i = handle_long(&mut args, &argv, i, long)?;
        } else if let Some(short_body) = s.strip_prefix('-') {
            if short_body.is_empty() {
                // bare `-` is a positional ("read from this filename")
                apply_positional(&mut args, raw, &mut positional_count)?;
                i += 1;
            } else {
                i = handle_short(&mut args, &argv, i, short_body)?;
            }
        } else {
            apply_positional(&mut args, raw, &mut positional_count)?;
            i += 1;
        }
    }
    args.finalize()?;
    Ok(args)
}

fn apply_positional(
    args: &mut Args,
    raw: &OsString,
    positional_count: &mut usize,
) -> Result<(), ParseError> {
    if *positional_count >= 1 {
        return Err(ParseError::unknown_argument(format!(
            "unexpected positional argument '{}'",
            raw.to_string_lossy()
        )));
    }
    args.file = Some(PathBuf::from(raw));
    *positional_count += 1;
    Ok(())
}

/// `finalize` translates raw clap-side inputs into the typed public
/// fields and runs the cross-flag validation that used to live in
/// `Args::finalize`. Kept here so [`parse_argv`] is the single entry
/// point that returns a fully-finalised `Args`.
impl Args {
    pub(super) fn finalize(&mut self) -> Result<(), ParseError> {
        use std::path::Path;

        use crate::cli::run_cli::derive_output_basename;

        let raw_plugins = std::mem::take(&mut self.plugin_occurrences);
        for occurrence in raw_plugins {
            for name in occurrence {
                if !self.plugins.contains(&name) {
                    self.plugins.push(name);
                }
            }
        }
        let raw_roots = std::mem::take(&mut self.raw_roots);
        for raw in raw_roots {
            match parse_root_queries(&raw) {
                Ok(qs) => self.roots.extend(qs),
                Err(msg) => return Err(ParseError::value_validation(msg)),
            }
        }
        let raw_highlight = std::mem::take(&mut self.raw_highlight);
        self.highlight = match raw_highlight {
            None => Highlight::Absent,
            Some(None) => Highlight::NoValue,
            Some(Some(raw)) => match parse_highlight_queries(&raw) {
                Ok(qs) => Highlight::Value(qs),
                Err(msg) => return Err(ParseError::value_validation(msg)),
            },
        };
        if self.out_dir.is_some() && self.out_file.is_some() {
            return Err(ParseError::argument_conflict(
                "--out-dir and --out-file cannot both be set",
            ));
        }
        if self.out_dir.is_some() {
            // `-o` writes to `<dir>/<auto-basename>.<ext>`, so it needs
            // either a positional file (basename comes from it) or at
            // least one `-r` query (basename comes from the root tokens).
            // `--stdin` removes the positional path, leaving `-r` as the
            // only remaining basename source.
            if self.stdin && self.roots.is_empty() {
                return Err(ParseError::value_validation(
                    "--out-dir requires either -r/--roots or an input file path",
                ));
            }
            let input_path = if self.stdin {
                Path::new("")
            } else {
                self.file.as_deref().unwrap_or_else(|| Path::new(""))
            };
            self.derived_basename = Some(derive_output_basename(
                &self.roots,
                self.descendants,
                self.ancestors,
                self.context,
                input_path,
            ));
        }
        Ok(())
    }
}
