use super::*;

use crate::cli::args::CliFormat;

fn capture(format: &CliFormat) -> String {
    let mut buf = Vec::new();
    emit_stats_deprecation_notice(format, &mut buf);
    String::from_utf8(buf).expect("output should be valid UTF-8")
}

#[test]
fn writes_nothing_for_non_stats_formats() {
    for format in [
        CliFormat::Mermaid,
        CliFormat::Ir,
        CliFormat::Json,
        CliFormat::Markdown,
    ] {
        assert_eq!(capture(&format), "");
    }
}

#[test]
fn writes_a_deprecation_warning_for_stats() {
    assert_eq!(
        capture(&CliFormat::Stats),
        "uns: warning: '-f stats' is deprecated and will be removed in a future release; the 'uns stats' subcommand will replace it.\n",
    );
}

#[test]
fn the_warning_stays_non_committal_about_the_removal_version() {
    let out = capture(&CliFormat::Stats);
    assert!(!out.contains("0.6.0"), "must not assert a specific version");
    assert!(out.contains("a future release"));
}
