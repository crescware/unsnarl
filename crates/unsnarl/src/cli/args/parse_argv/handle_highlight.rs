//! `--highlight` / `-H` optional-value flag handling for parse_argv.

use std::ffi::OsString;

use crate::cli::args::parse_error::ParseError;
use crate::cli::args::Args;

use super::values::take_optional_value;

/// `--highlight` shape: `--highlight=v` → Value, `--highlight v` →
/// Value if `v` doesn't look like a flag, else NoValue.
pub(super) fn handle_highlight_long(
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
pub(super) fn handle_highlight_short(
    args: &mut Args,
    argv: &[OsString],
    i: usize,
    attached_value: Option<String>,
) -> Result<usize, ParseError> {
    let (value_opt, next_i) = take_optional_value(argv, i, attached_value);
    args.raw_highlight = Some(value_opt);
    Ok(next_i)
}
