//! CLI argument parsing error type.
//!
//! Replaces the surface area of `clap::Error` that `args_test.rs` and
//! `run.rs` depend on: an `exit_code()` for `main()`'s `ExitCode`, a
//! `kind()` enum for the tests to distinguish `--help` / `--version`
//! from genuine usage errors, and an `exit()` shortcut that mimics
//! clap's `Error::exit()` (writing the rendered text to the
//! appropriate stream, then `std::process::exit`).
//!
//! Behaviour intentionally preserved from the clap surface:
//! - `--help` / `--version` produce an error whose `exit_code()` is 0
//!   and whose rendered text goes to stdout. The "error" framing keeps
//!   the early-exit control flow identical to the previous
//!   `clap::Error` path.
//! - Every other variant has exit code 2 and renders to stderr.
//! - `Display` for the error renders the user-facing message body
//!   (without the trailing newline) so callers compose their own
//!   formatting around it.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// `--help` / `-h` was passed. Renders the help text and exits 0.
    DisplayHelp,
    /// `--version` / `-v` was passed. Renders the version string and exits 0.
    DisplayVersion,
    /// An unrecognised flag appeared in argv.
    UnknownArgument,
    /// A flag's value was not one of the accepted choices (`-f`, `--color-theme`, etc.).
    InvalidValue,
    /// A `value_parser`-equivalent rejected the supplied value
    /// (`-A abc`, `--depth -1`, `--plugin vue`, `-r foo-bar`, ...).
    ValueValidation,
    /// A flag required a value and none was supplied (e.g. `--format` with no following token).
    MissingValue,
    /// Two mutually exclusive flags were both passed (`--out-dir` + `--out-file`).
    ArgumentConflict,
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    message: String,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn display_help(message: impl Into<String>) -> Self {
        Self::new(ParseErrorKind::DisplayHelp, message)
    }

    pub fn display_version(message: impl Into<String>) -> Self {
        Self::new(ParseErrorKind::DisplayVersion, message)
    }

    pub fn unknown_argument(message: impl Into<String>) -> Self {
        Self::new(ParseErrorKind::UnknownArgument, message)
    }

    pub fn invalid_value(message: impl Into<String>) -> Self {
        Self::new(ParseErrorKind::InvalidValue, message)
    }

    pub fn value_validation(message: impl Into<String>) -> Self {
        Self::new(ParseErrorKind::ValueValidation, message)
    }

    pub fn missing_value(message: impl Into<String>) -> Self {
        Self::new(ParseErrorKind::MissingValue, message)
    }

    pub fn argument_conflict(message: impl Into<String>) -> Self {
        Self::new(ParseErrorKind::ArgumentConflict, message)
    }

    pub fn kind(&self) -> ParseErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    /// Same convention as `clap::Error::exit_code()`: 0 for the
    /// help/version "errors", 2 for genuine usage failures.
    pub fn exit_code(&self) -> i32 {
        match self.kind {
            ParseErrorKind::DisplayHelp | ParseErrorKind::DisplayVersion => 0,
            _ => 2,
        }
    }

    /// Mirror of `clap::Error::exit`: write the rendered message to
    /// stdout for help/version, stderr otherwise, then exit with
    /// [`Self::exit_code`]. Used by the `Args::parse` shortcut, which
    /// is itself the inherent twin of the clap-derived
    /// `<Args as Parser>::parse` that previously delegated to
    /// `Error::exit`.
    pub fn exit(&self) -> ! {
        match self.kind {
            ParseErrorKind::DisplayHelp | ParseErrorKind::DisplayVersion => {
                print!("{}", self.message);
            }
            _ => {
                eprintln!("error: {}", self.message);
            }
        }
        std::process::exit(self.exit_code());
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ParseError {}
