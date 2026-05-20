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

fn first_line(out: &str) -> &str {
    out.lines().next().expect("at least one line")
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
    let out = capture_stdout(&["uns", "-f", "stats", "x.ts"]);
    assert_eq!(first_line(&out), "Not implemented yet: stats emitter");
}

#[test]
fn unknown_format_is_rejected_by_clap_before_dispatch() {
    let err = Args::try_parse_from(["uns", "-f", "bogus", "x.ts"]).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn stub_emitter_output_includes_parsed_args_json_after_label() {
    // `stats` is still a stub; the legacy "Not implemented yet" line
    // + CLI args JSON shape lives on through the unimplemented formats.
    let out = capture_stdout(&["uns", "-f", "stats", "x.ts"]);
    let (label, rest) = out.split_once('\n').expect("label line");
    assert_eq!(label, "Not implemented yet: stats emitter");
    let value: serde_json::Value =
        serde_json::from_str(rest.trim_end()).expect("rest should be JSON");
    assert_eq!(value["format"], "stats");
    assert_eq!(value["file"], "x.ts");
}

#[test]
fn out_flag_notice_is_emitted_to_stderr_when_out_dir_basename_has_an_extension() {
    // Pair the `-o graph.mmd` notice with `-f stats` so the test
    // does not need a real input file -- the notice fires during arg
    // finalize, before the unimplemented emitter prints its stub body.
    let (out, err) = capture(&["uns", "-f", "stats", "-o", "graph.mmd", "x.ts"]);
    assert_eq!(
        first_line(&err),
        "uns: notice: -o 'graph.mmd' is treated as a directory name; use --out-file to write to that path as a file."
    );
    // The emitter output still lands on stdout; the notice does not
    // pollute it.
    assert_eq!(first_line(&out), "Not implemented yet: stats emitter");
}

#[test]
fn out_flag_notice_is_not_emitted_for_extensionless_out_dir() {
    let (out, err) = capture(&["uns", "-f", "stats", "-o", "build", "x.ts"]);
    assert_eq!(err, "");
    assert_eq!(first_line(&out), "Not implemented yet: stats emitter");
}
