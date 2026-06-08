use std::io::Write;

use crate::cli::args::CliFormat;

/// Warn on stderr that `-f stats` is deprecated and slated for
/// removal, steering users toward the forthcoming `uns stats`
/// subcommand that will replace it. Fires only for the stats format;
/// every other emitter stays silent. The removal is announced as "a
/// future release" rather than a specific version, so the timeline
/// stays non-committal.
pub fn emit_stats_deprecation_notice(format: &CliFormat, stderr: &mut dyn Write) {
    if !matches!(format, CliFormat::Stats) {
        return;
    }
    let _ = writeln!(
        stderr,
        "uns: warning: '-f stats' is deprecated and will be removed in a future release; the 'uns stats' subcommand will replace it."
    );
}

#[cfg(test)]
#[path = "emit_stats_deprecation_notice_test.rs"]
mod emit_stats_deprecation_notice_test;
