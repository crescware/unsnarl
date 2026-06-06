//! Short-form (`-X` / `-X value` / `-Xvalue` / `-X=value`) flag
//! handling for parse_argv.

use std::ffi::OsString;
use std::path::PathBuf;

use crate::cli::args::cli_format::CliFormat;
use crate::cli::args::parse_error::ParseError;
use crate::cli::args::parse_generation_count::parse_generation_count;
use crate::cli::args::Args;

use super::handle_highlight::handle_highlight_short;
use super::values::{invalid_value, require_value};

/// Short-form (`-X` / `-X value` / `-Xvalue` / `-X=value`). Returns
/// the new argv index.
pub(super) fn handle_short(
    args: &mut Args,
    argv: &[OsString],
    i: usize,
    body: &str,
) -> Result<usize, ParseError> {
    let mut chars = body.chars();
    let name = chars
        .next()
        .expect("caller gates handle_short on a non-empty short body (parse_argv loop)");
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
