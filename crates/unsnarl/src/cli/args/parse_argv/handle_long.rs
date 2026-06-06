//! Long-form (`--name` / `--name=value` / `--name value`) flag
//! handling for parse_argv.

use std::ffi::OsString;
use std::path::PathBuf;

use crate::cli::args::cli_color_theme::CliColorTheme;
use crate::cli::args::cli_format::CliFormat;
use crate::cli::args::cli_language::CliLanguage;
use crate::cli::args::cli_mermaid_renderer::CliMermaidRenderer;
use crate::cli::args::collect_plugins::parse_plugin_occurrence;
use crate::cli::args::parse_error::ParseError;
use crate::cli::args::parse_generation_count::{parse_generation_count, parse_nesting_depth};
use crate::cli::args::Args;

use super::handle_highlight::handle_highlight_long;
use super::values::{forbid_value, invalid_value, require_value};

/// Long-form (`--name` / `--name=value` / `--name value`). Returns
/// the new argv index.
pub(super) fn handle_long(
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
