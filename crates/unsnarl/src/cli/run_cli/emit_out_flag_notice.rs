use std::io::Write;
use std::path::Path;

/// Tell the user when `-o foo.json` would be silently treated as a
/// directory name. The dot in the basename is the heuristic for
/// "looks like a filename"; an empty extname means no notice.
pub fn emit_out_flag_notice(out_dir: Option<&Path>, stderr: &mut dyn Write) {
    let Some(path) = out_dir else {
        return;
    };
    if path.extension().is_none() {
        return;
    }
    let _ = writeln!(
        stderr,
        "uns: notice: -o '{}' is treated as a directory name; use --out-file to write to that path as a file.",
        path.display()
    );
}

#[cfg(test)]
#[path = "emit_out_flag_notice_test.rs"]
mod emit_out_flag_notice_test;
