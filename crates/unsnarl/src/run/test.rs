use super::*;

fn capture(argv: &[&str]) -> String {
    let args = Args::try_parse_argv(argv).expect("argv should parse");
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
fn ir_format_routes_to_ir_emitter() {
    let out = capture(&["uns", "-f", "ir", "x.ts"]);
    assert_eq!(first_line(&out), "Not implemented yet: ir emitter");
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
    let err = Args::try_parse_argv(["uns", "-f", "bogus", "x.ts"]).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn emitter_output_includes_parsed_args_json_after_label() {
    let out = capture(&["uns", "-f", "ir", "x.ts"]);
    let (label, rest) = out.split_once('\n').expect("label line");
    assert_eq!(label, "Not implemented yet: ir emitter");
    let value: serde_json::Value =
        serde_json::from_str(rest.trim_end()).expect("rest should be JSON");
    assert_eq!(value["format"], "ir");
    assert_eq!(value["file"], "x.ts");
}
