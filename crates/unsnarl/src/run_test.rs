use super::*;

fn capture(argv: &[&str]) -> String {
    let args = Args::try_parse_from(argv).expect("argv should parse");
    let mut buf = Vec::new();
    run_to(&args, &mut buf);
    String::from_utf8(buf).expect("output should be valid UTF-8")
}

fn first_line(out: &str) -> &str {
    out.lines().next().expect("at least one line")
}

#[test]
fn default_format_routes_to_mermaid_emitter() {
    let out = capture(&["uns", "x.ts"]);
    assert_eq!(first_line(&out), "Not implemented yet: mermaid emitter");
}

#[test]
fn mermaid_format_routes_to_mermaid_emitter() {
    let out = capture(&["uns", "-f", "mermaid", "x.ts"]);
    assert_eq!(first_line(&out), "Not implemented yet: mermaid emitter");
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
    let out = capture(&["uns", "-f", "ir", &path]);
    let value: serde_json::Value =
        serde_json::from_str(out.trim_end()).expect("ir emitter output should be JSON");
    assert_eq!(value["version"], 1);
    assert_eq!(value["source"]["language"], "ts");
}

#[test]
fn json_format_routes_to_json_emitter() {
    let out = capture(&["uns", "-f", "json", "x.ts"]);
    assert_eq!(first_line(&out), "Not implemented yet: json emitter");
}

#[test]
fn markdown_format_routes_to_markdown_emitter() {
    let out = capture(&["uns", "-f", "markdown", "x.ts"]);
    assert_eq!(first_line(&out), "Not implemented yet: markdown emitter");
}

#[test]
fn stats_format_routes_to_stats_emitter() {
    let out = capture(&["uns", "-f", "stats", "x.ts"]);
    assert_eq!(first_line(&out), "Not implemented yet: stats emitter");
}

#[test]
fn unknown_format_is_rejected_by_clap_before_dispatch() {
    let err = Args::try_parse_from(["uns", "-f", "bogus", "x.ts"]).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn stub_emitter_output_includes_parsed_args_json_after_label() {
    // `json` is still a stub; the legacy "Not implemented yet" line +
    // CLI args JSON shape lives on through the unimplemented formats.
    let out = capture(&["uns", "-f", "json", "x.ts"]);
    let (label, rest) = out.split_once('\n').expect("label line");
    assert_eq!(label, "Not implemented yet: json emitter");
    let value: serde_json::Value =
        serde_json::from_str(rest.trim_end()).expect("rest should be JSON");
    assert_eq!(value["format"], "json");
    assert_eq!(value["file"], "x.ts");
}

#[test]
fn out_flag_notice_is_emitted_when_out_dir_basename_has_an_extension() {
    let out = capture(&["uns", "-o", "graph.mmd", "x.ts"]);
    let first = first_line(&out);
    assert_eq!(
        first,
        "uns: notice: -o 'graph.mmd' is treated as a directory name; use --out-file to write to that path as a file."
    );
}

#[test]
fn out_flag_notice_is_not_emitted_for_extensionless_out_dir() {
    let out = capture(&["uns", "-o", "build", "x.ts"]);
    assert_eq!(first_line(&out), "Not implemented yet: mermaid emitter");
}
