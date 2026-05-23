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

use unsnarl_root_query::parse_root_queries;

use super::cli_color_theme::CliColorTheme;
use super::cli_format::CliFormat;
use super::cli_language::CliLanguage;
use super::cli_mermaid_renderer::CliMermaidRenderer;
use super::collect_plugins::parse_plugin_occurrence;
use super::help_text::{version_text, HELP_TEXT};
use super::highlight::Highlight;
use super::parse_error::ParseError;
use super::parse_generation_count::{parse_generation_count, parse_nesting_depth};
use super::Args;

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

/// Long-form (`--name` / `--name=value` / `--name value`). Returns
/// the new argv index.
fn handle_long(
    args: &mut Args,
    argv: &[OsString],
    i: usize,
    rest: &str,
) -> Result<usize, ParseError> {
    let (name, inline_value) = match rest.split_once('=') {
        Some((n, v)) => (n, Some(v.to_string())),
        None => (rest, None),
    };
    match name {
        // Boolean flags
        "no-pretty-json" => {
            forbid_value(name, inline_value.as_deref())?;
            args.pretty_json = false;
            Ok(i + 1)
        }
        "stdin" => {
            forbid_value(name, inline_value.as_deref())?;
            args.stdin = true;
            Ok(i + 1)
        }
        "debug" => {
            forbid_value(name, inline_value.as_deref())?;
            args.debug = true;
            Ok(i + 1)
        }
        "verbose" => {
            forbid_value(name, inline_value.as_deref())?;
            args.verbose = true;
            Ok(i + 1)
        }

        // Value-taking flags
        "format" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.format = CliFormat::parse(&value).ok_or_else(|| invalid_value(name, &value))?;
            Ok(next_i)
        }
        "mermaid-renderer" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.mermaid_renderer =
                Some(CliMermaidRenderer::parse(&value).ok_or_else(|| invalid_value(name, &value))?);
            Ok(next_i)
        }
        "color-theme" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.color_theme =
                CliColorTheme::parse(&value).ok_or_else(|| invalid_value(name, &value))?;
            Ok(next_i)
        }
        "stdin-lang" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.stdin_lang =
                CliLanguage::parse(&value).ok_or_else(|| invalid_value(name, &value))?;
            Ok(next_i)
        }
        "roots" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.raw_roots.push(value);
            Ok(next_i)
        }
        "highlight" => Ok(handle_highlight_long(args, argv, i, inline_value)?),
        "descendants" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.descendants =
                Some(parse_generation_count(&value).map_err(ParseError::value_validation)?);
            Ok(next_i)
        }
        "ancestors" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.ancestors =
                Some(parse_generation_count(&value).map_err(ParseError::value_validation)?);
            Ok(next_i)
        }
        "context" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.context =
                Some(parse_generation_count(&value).map_err(ParseError::value_validation)?);
            Ok(next_i)
        }
        "depth" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.depth = Some(parse_nesting_depth(&value).map_err(ParseError::value_validation)?);
            Ok(next_i)
        }
        "depth-function" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.depth_function =
                Some(parse_nesting_depth(&value).map_err(ParseError::value_validation)?);
            Ok(next_i)
        }
        "depth-block" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.depth_block =
                Some(parse_nesting_depth(&value).map_err(ParseError::value_validation)?);
            Ok(next_i)
        }
        "out-dir" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.out_dir = Some(PathBuf::from(value));
            Ok(next_i)
        }
        "out-file" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            args.out_file = Some(PathBuf::from(value));
            Ok(next_i)
        }
        "plugin" => {
            let (value, next_i) = require_value(argv, i, name, inline_value)?;
            let plugins = parse_plugin_occurrence(&value).map_err(ParseError::value_validation)?;
            args.plugin_occurrences.push(plugins);
            Ok(next_i)
        }

        _ => Err(ParseError::unknown_argument(format!(
            "unrecognised argument '--{name}'"
        ))),
    }
}

/// Short-form (`-X` / `-X value` / `-Xvalue` / `-X=value`). Returns
/// the new argv index.
fn handle_short(
    args: &mut Args,
    argv: &[OsString],
    i: usize,
    body: &str,
) -> Result<usize, ParseError> {
    let mut chars = body.chars();
    let name = chars.next().unwrap();
    let tail: String = chars.collect();
    // Attached value: `-XV` → V is value, `-X=V` → V is value.
    let attached_value: Option<String> = if tail.is_empty() {
        None
    } else if let Some(eq_trim) = tail.strip_prefix('=') {
        Some(eq_trim.to_string())
    } else {
        Some(tail)
    };
    match name {
        'f' => {
            let (value, next_i) = require_value(argv, i, "f", attached_value)?;
            args.format = CliFormat::parse(&value).ok_or_else(|| invalid_value("f", &value))?;
            Ok(next_i)
        }
        'r' => {
            let (value, next_i) = require_value(argv, i, "r", attached_value)?;
            args.raw_roots.push(value);
            Ok(next_i)
        }
        'H' => Ok(handle_highlight_short(args, argv, i, attached_value)?),
        'A' => {
            let (value, next_i) = require_value(argv, i, "A", attached_value)?;
            args.descendants =
                Some(parse_generation_count(&value).map_err(ParseError::value_validation)?);
            Ok(next_i)
        }
        'B' => {
            let (value, next_i) = require_value(argv, i, "B", attached_value)?;
            args.ancestors =
                Some(parse_generation_count(&value).map_err(ParseError::value_validation)?);
            Ok(next_i)
        }
        'C' => {
            let (value, next_i) = require_value(argv, i, "C", attached_value)?;
            args.context =
                Some(parse_generation_count(&value).map_err(ParseError::value_validation)?);
            Ok(next_i)
        }
        'o' => {
            let (value, next_i) = require_value(argv, i, "o", attached_value)?;
            args.out_dir = Some(PathBuf::from(value));
            Ok(next_i)
        }
        _ => Err(ParseError::unknown_argument(format!(
            "unrecognised argument '-{name}'"
        ))),
    }
}

/// `--highlight` shape: `--highlight=v` → Value, `--highlight v` →
/// Value if `v` doesn't look like a flag, else NoValue.
fn handle_highlight_long(
    args: &mut Args,
    argv: &[OsString],
    i: usize,
    inline_value: Option<String>,
) -> Result<usize, ParseError> {
    let (value_opt, next_i) = take_optional_value(argv, i, inline_value);
    args.raw_highlight = Some(value_opt);
    Ok(next_i)
}

/// `-H` shape: `-H=v` / `-Hv` → Value, `-H v` → Value if v doesn't
/// look like a flag, `-H` (alone / followed by flag-looking token)
/// → NoValue.
fn handle_highlight_short(
    args: &mut Args,
    argv: &[OsString],
    i: usize,
    attached_value: Option<String>,
) -> Result<usize, ParseError> {
    let (value_opt, next_i) = take_optional_value(argv, i, attached_value);
    args.raw_highlight = Some(value_opt);
    Ok(next_i)
}

/// Shared between the long and short forms of an optional-value
/// flag. Consumes the attached form when present, otherwise the next
/// argv token IFF it's not a flag.
fn take_optional_value(
    argv: &[OsString],
    i: usize,
    inline_value: Option<String>,
) -> (Option<String>, usize) {
    if inline_value.is_some() {
        return (inline_value, i + 1);
    }
    match argv.get(i + 1).and_then(|raw| raw.to_str()) {
        Some(next) if !looks_like_flag(next) => (Some(next.to_string()), i + 2),
        _ => (None, i + 1),
    }
}

fn require_value(
    argv: &[OsString],
    i: usize,
    name: &str,
    inline_value: Option<String>,
) -> Result<(String, usize), ParseError> {
    if let Some(v) = inline_value {
        return Ok((v, i + 1));
    }
    match argv.get(i + 1) {
        Some(raw) => {
            let Some(s) = raw.to_str() else {
                return Err(ParseError::missing_value(format!(
                    "value for '{name}' is not valid UTF-8"
                )));
            };
            if looks_like_flag(s) {
                Err(ParseError::missing_value(format!(
                    "missing value for '{name}'"
                )))
            } else {
                Ok((s.to_string(), i + 2))
            }
        }
        None => Err(ParseError::missing_value(format!(
            "missing value for '{name}'"
        ))),
    }
}

fn forbid_value(name: &str, inline_value: Option<&str>) -> Result<(), ParseError> {
    if inline_value.is_some() {
        return Err(ParseError::invalid_value(format!(
            "'{name}' does not take a value"
        )));
    }
    Ok(())
}

/// Counts a token as a flag if it begins with `-` and is not the
/// single `-` (which is a conventional alias for "stdin file"). Used
/// to keep `-A -1` from accidentally treating `-1` as a value.
fn looks_like_flag(s: &str) -> bool {
    s.starts_with('-') && s != "-"
}

fn invalid_value(name: &str, value: &str) -> ParseError {
    let accepted = accepted_values(name);
    let suffix = match accepted {
        Some(values) => format!(" (expected one of: {})", values.join(", ")),
        None => String::new(),
    };
    ParseError::invalid_value(format!("invalid value '{value}' for '{name}'{suffix}"))
}

fn accepted_values(name: &str) -> Option<&'static [&'static str]> {
    match name {
        "f" | "format" => Some(CliFormat::ACCEPTED),
        "mermaid-renderer" => Some(CliMermaidRenderer::ACCEPTED),
        "color-theme" => Some(CliColorTheme::ACCEPTED),
        "stdin-lang" => Some(CliLanguage::ACCEPTED),
        _ => None,
    }
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
            Some(Some(raw)) => match parse_root_queries(&raw) {
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
