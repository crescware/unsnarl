//! Value-consumption helpers shared by the long / short flag handlers:
//! optional vs required value extraction, the flag-look heuristic, and
//! the invalid-value error builder.

use std::ffi::OsString;

use crate::cli::args::cli_color_theme::CliColorTheme;
use crate::cli::args::cli_format::CliFormat;
use crate::cli::args::cli_language::CliLanguage;
use crate::cli::args::cli_mermaid_renderer::CliMermaidRenderer;
use crate::cli::args::parse_error::ParseError;

/// Shared between the long and short forms of an optional-value
/// flag. Consumes the attached form when present, otherwise the next
/// argv token IFF it's not a flag.
pub(super) fn take_optional_value(
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

pub(super) fn require_value(
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

pub(super) fn forbid_value(name: &str, inline_value: Option<&str>) -> Result<(), ParseError> {
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

pub(super) fn invalid_value(name: &str, value: &str) -> ParseError {
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
