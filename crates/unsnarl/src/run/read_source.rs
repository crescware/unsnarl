//! Materialise the source bytes a CLI run analyses, with the
//! path / language pair the emitter records.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use unsnarl_ir::Language;

use crate::cli::args::CliLanguage;
use crate::cli::run_cli::calc_source::ExecuteSource;
use crate::pipeline::language_for_path;

/// Materialise the source bytes the pipeline runs against, together
/// with the path / language pair the emitter records inside
/// `SerializedSource`. Stdin contents are labelled `stdin.<lang>` so
/// the IR carries a stable, lang-aware path; file inputs map to the
/// on-disk path and the extension drives language detection.
pub(super) fn read_source_text(
    src: &ExecuteSource,
    err: &mut dyn Write,
) -> Option<(String, String, Language)> {
    match src {
        ExecuteSource::Stdin { text, lang } => {
            let language = language_for_cli(lang);
            let source_path = format!("stdin.{}", cli_language_str(lang));
            Some((text.clone(), source_path, language))
        }
        ExecuteSource::File { path } => read_source_file(path, err),
    }
}

fn read_source_file(path: &PathBuf, err: &mut dyn Write) -> Option<(String, String, Language)> {
    let code = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            writeln!(err, "error: failed to read {}: {e}", path.display()).ok();
            return None;
        }
    };
    let source_path = path.to_string_lossy().into_owned();
    let Some(language) = language_for_path(&source_path) else {
        let ext = Path::new(&source_path)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("(none)");
        writeln!(err, "error: unsupported language extension: {ext}").ok();
        return None;
    };
    Some((code, source_path, language))
}

fn language_for_cli(lang: &CliLanguage) -> Language {
    match lang {
        CliLanguage::Ts => Language::Ts,
        CliLanguage::Tsx => Language::Tsx,
        CliLanguage::Js => Language::Js,
        CliLanguage::Jsx => Language::Jsx,
    }
}

fn cli_language_str(lang: &CliLanguage) -> &'static str {
    match lang {
        CliLanguage::Ts => "ts",
        CliLanguage::Tsx => "tsx",
        CliLanguage::Js => "js",
        CliLanguage::Jsx => "jsx",
    }
}
