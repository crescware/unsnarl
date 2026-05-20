use super::*;

/// Run the CLI and return `(stdout, stderr)` as strings.
fn capture(argv: &[&str]) -> (String, String) {
    let args = Args::try_parse_from(argv).expect("argv should parse");
    let mut out = Vec::new();
    let mut err = Vec::new();
    run_to(&args, &mut out, &mut err);
    (
        String::from_utf8(out).expect("stdout should be valid UTF-8"),
        String::from_utf8(err).expect("stderr should be valid UTF-8"),
    )
}

/// Convenience wrapper that asserts stderr is empty and returns stdout.
fn capture_stdout(argv: &[&str]) -> String {
    let (out, err) = capture(argv);
    assert_eq!(err, "", "expected empty stderr, got: {err}");
    out
}

#[test]
fn default_format_routes_to_mermaid_emitter() {
    use std::io::Write;
    let mut tmp = tempfile::Builder::new()
        .suffix(".ts")
        .tempfile()
        .expect("create tempfile");
    writeln!(tmp, "let x = 1;").expect("write tempfile");
    let path = tmp
        .path()
        .to_str()
        .expect("tempfile path utf-8")
        .to_string();
    let out = capture_stdout(&["uns", &path]);
    // The default (`-f mermaid`) emits the elk preamble first, then
    // `flowchart RL`. Asserting on the first two lines is enough to
    // distinguish the real emitter from the legacy stub output.
    let mut lines = out.lines();
    assert_eq!(
        lines.next().expect("preamble line"),
        r#"%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%"#
    );
    assert_eq!(lines.next().expect("flowchart line"), "flowchart RL");
}

#[test]
fn mermaid_format_routes_to_mermaid_emitter() {
    use std::io::Write;
    let mut tmp = tempfile::Builder::new()
        .suffix(".ts")
        .tempfile()
        .expect("create tempfile");
    writeln!(tmp, "let x = 1;").expect("write tempfile");
    let path = tmp
        .path()
        .to_str()
        .expect("tempfile path utf-8")
        .to_string();
    let out = capture_stdout(&["uns", "-f", "mermaid", &path]);
    let mut lines = out.lines();
    assert_eq!(
        lines.next().expect("preamble line"),
        r#"%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%"#
    );
    assert_eq!(lines.next().expect("flowchart line"), "flowchart RL");
}

#[test]
fn mermaid_dagre_renderer_omits_the_elk_init_directive() {
    use std::io::Write;
    let mut tmp = tempfile::Builder::new()
        .suffix(".ts")
        .tempfile()
        .expect("create tempfile");
    writeln!(tmp, "let x = 1;").expect("write tempfile");
    let path = tmp
        .path()
        .to_str()
        .expect("tempfile path utf-8")
        .to_string();
    let out = capture_stdout(&["uns", "-f", "mermaid", "--mermaid-renderer", "dagre", &path]);
    // dagre carries no preamble lines, so the very first emitted
    // line is `flowchart RL`.
    assert_eq!(out.lines().next().expect("flowchart line"), "flowchart RL");
}

#[test]
fn ir_format_emits_ir_json_for_a_real_file() {
    use std::io::Write;
    let mut tmp = tempfile::Builder::new()
        .suffix(".ts")
        .tempfile()
        .expect("create tempfile");
    writeln!(tmp, "let x = 1;").expect("write tempfile");
    let path = tmp
        .path()
        .to_str()
        .expect("tempfile path utf-8")
        .to_string();
    let out = capture_stdout(&["uns", "-f", "ir", &path]);
    let value: serde_json::Value =
        serde_json::from_str(out.trim_end()).expect("ir emitter output should be JSON");
    assert_eq!(value["version"], 1);
    assert_eq!(value["source"]["language"], "ts");
}

#[test]
fn json_format_emits_visual_graph_json_for_a_real_file() {
    use std::io::Write;
    let mut tmp = tempfile::Builder::new()
        .suffix(".ts")
        .tempfile()
        .expect("create tempfile");
    writeln!(tmp, "let x = 1;").expect("write tempfile");
    let path = tmp
        .path()
        .to_str()
        .expect("tempfile path utf-8")
        .to_string();
    let out = capture_stdout(&["uns", "-f", "json", &path]);
    let value: serde_json::Value =
        serde_json::from_str(out.trim_end()).expect("json emitter output should be JSON");
    assert_eq!(value["version"], 1);
    assert_eq!(value["source"]["language"], "ts");
    assert_eq!(value["direction"], "RL");
    assert!(value["elements"].is_array(), "elements should be an array");
    assert!(value["edges"].is_array(), "edges should be an array");
}

#[test]
fn markdown_format_routes_to_markdown_emitter() {
    use std::io::Write;
    let mut tmp = tempfile::Builder::new()
        .suffix(".ts")
        .tempfile()
        .expect("create tempfile");
    writeln!(tmp, "let x = 1;").expect("write tempfile");
    let path = tmp
        .path()
        .to_str()
        .expect("tempfile path utf-8")
        .to_string();
    let out = capture_stdout(&["uns", "-f", "markdown", &path]);
    // The markdown emitter renders `# <path>` as the very first line,
    // then a blank line and the `## Input` section. Asserting on the
    // first two lines is enough to distinguish the real emitter from
    // the legacy stub output.
    let mut lines = out.lines();
    assert_eq!(lines.next().expect("title line"), format!("# {path}"));
    assert_eq!(lines.next().expect("blank line"), "");
}

#[test]
fn stats_format_routes_to_stats_emitter() {
    use std::io::Write;
    let mut tmp = tempfile::Builder::new()
        .suffix(".ts")
        .tempfile()
        .expect("create tempfile");
    writeln!(tmp, "let x = 1;").expect("write tempfile");
    let path = tmp
        .path()
        .to_str()
        .expect("tempfile path utf-8")
        .to_string();
    let out = capture_stdout(&["uns", "-f", "stats", &path]);
    // The stats emitter writes a TSV row per node followed by a
    // `<N> total` summary; asserting on the trailing "total" line
    // is enough to distinguish the real emitter from the legacy
    // stub output.
    let last = out.lines().last().expect("at least one line");
    assert!(
        last.ends_with(" total"),
        "expected trailing total row, got: {last}"
    );
}

#[test]
fn unknown_format_is_rejected_by_clap_before_dispatch() {
    let err = Args::try_parse_from(["uns", "-f", "bogus", "x.ts"]).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn out_flag_notice_is_emitted_to_stderr_when_out_dir_basename_has_an_extension() {
    // `-o graph.mmd` triggers the notice during arg finalize, before
    // any emitter runs. Pair it with an unreadable input path so the
    // emitter exits early via its `read_source` error branch -- the
    // notice still has to surface on stderr regardless.
    let (_out, err) = capture(&["uns", "-f", "stats", "-o", "graph.mmd", "x.ts"]);
    let first_err_line = err.lines().next().expect("at least one stderr line");
    assert_eq!(
        first_err_line,
        "uns: notice: -o 'graph.mmd' is treated as a directory name; use --out-file to write to that path as a file."
    );
}

#[test]
fn out_flag_notice_is_not_emitted_for_extensionless_out_dir() {
    // No notice in stderr means only the emitter's own error (the
    // input file does not exist) survives. Assert that the notice
    // line is absent rather than insisting on a fully empty stderr.
    let (_out, err) = capture(&["uns", "-f", "stats", "-o", "build", "x.ts"]);
    assert!(
        !err.contains("uns: notice:"),
        "expected no -o notice, got stderr: {err}"
    );
}
