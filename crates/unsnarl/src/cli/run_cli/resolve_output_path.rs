//! Resolve the output path the emitted text should land on.
//!
//! `--out-file <path>` is returned verbatim. `--out-dir <dir>`
//! combines the pre-derived basename (computed by `Args::finalize`
//! from `-r` tokens or the input file stem) with the active
//! emitter's `extension` to produce `<dir>/<basename>.<ext>`.

use std::path::PathBuf;

use unsnarl_emitter::Emitter;

use crate::cli::args::Args;

pub mod derive_output_basename;
pub mod radius_suffix;
pub mod root_query_token;

pub use derive_output_basename::derive_output_basename;

/// Resolve the output destination encoded in `args.out_file` /
/// `args.out_dir`. Returns `None` when neither flag was given (the
/// caller writes to stdout). The `--stdin && roots.empty()` guard
/// against `--out-dir` without a basename source is enforced at clap
/// parse time inside `Args::finalize`, so callers never see that
/// path through this resolver.
pub fn resolve_output_path(args: &Args, emitter: &dyn Emitter) -> Option<PathBuf> {
    if let Some(path) = args.out_file.as_ref() {
        return Some(path.clone());
    }
    let dir = args.out_dir.as_ref()?;
    let basename = args.derived_basename.as_deref().unwrap_or("");
    Some(dir.join(format!("{basename}.{}", emitter.extension())))
}

#[cfg(test)]
#[path = "resolve_output_path_test.rs"]
mod resolve_output_path_test;
