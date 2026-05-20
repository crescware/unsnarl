use super::*;
use unsnarl_ir::NestingDepth;

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

// `pruning_from_args` mirrors `resolveGenerations` in
// `ts/src/cli/run-cli/resolve-generations.ts`. The cases below port
// `ts/src/cli/run-cli/resolve-generations.test.ts` 1:1 plus the
// no-roots guard (TS expresses that guard in `runDetailed`; here it
// is folded into `pruning_from_args` so the parity is verified at
// the same seam).
//
// `parse_with` builds an `Args` from a synthetic argv so we exercise
// the same clap parser the binary uses (including the `-r` query
// parser and the `-A/-B/-C` value parsers).

fn parse_with(argv: &[&str]) -> Args {
    Args::try_parse_from(argv).expect("argv should parse")
}

#[test]
fn pruning_from_args_returns_none_when_no_roots_are_given() {
    let args = parse_with(&["uns", "x.ts"]);
    assert!(pruning_from_args(&args).is_none());
}

#[test]
fn pruning_from_args_no_radius_flag_yields_symmetric_default_generations() {
    let args = parse_with(&["uns", "-r", "render", "x.ts"]);
    let p = pruning_from_args(&args).expect("pruning options");
    assert_eq!(p.descendants, DEFAULT_GENERATIONS);
    assert_eq!(p.ancestors, DEFAULT_GENERATIONS);
}

#[test]
fn pruning_from_args_only_a_falls_other_side_to_zero() {
    let args = parse_with(&["uns", "-r", "render", "-A", "3", "x.ts"]);
    let p = pruning_from_args(&args).expect("pruning options");
    assert_eq!(p.descendants, 3);
    assert_eq!(p.ancestors, 0);
}

#[test]
fn pruning_from_args_only_b_falls_other_side_to_zero() {
    let args = parse_with(&["uns", "-r", "render", "-B", "4", "x.ts"]);
    let p = pruning_from_args(&args).expect("pruning options");
    assert_eq!(p.descendants, 0);
    assert_eq!(p.ancestors, 4);
}

#[test]
fn pruning_from_args_only_c_applies_to_both_sides() {
    let args = parse_with(&["uns", "-r", "render", "-C", "5", "x.ts"]);
    let p = pruning_from_args(&args).expect("pruning options");
    assert_eq!(p.descendants, 5);
    assert_eq!(p.ancestors, 5);
}

#[test]
fn pruning_from_args_c_plus_a_only_lets_b_inherit_context() {
    let args = parse_with(&["uns", "-r", "render", "-A", "1", "-C", "5", "x.ts"]);
    let p = pruning_from_args(&args).expect("pruning options");
    assert_eq!(p.descendants, 1);
    assert_eq!(p.ancestors, 5);
}

#[test]
fn pruning_from_args_explicit_a_and_b_make_c_irrelevant() {
    let args = parse_with(&[
        "uns", "-r", "render", "-A", "1", "-B", "2", "-C", "99", "x.ts",
    ]);
    let p = pruning_from_args(&args).expect("pruning options");
    assert_eq!(p.descendants, 1);
    assert_eq!(p.ancestors, 2);
}

#[test]
fn pruning_from_args_zero_is_explicit_not_falsy() {
    // `-A 0` says "no descendants generations"; the unspecified `-B`
    // still falls to 0 per the grep-style asymmetric rule.
    let args = parse_with(&["uns", "-r", "render", "-A", "0", "x.ts"]);
    let p = pruning_from_args(&args).expect("pruning options");
    assert_eq!(p.descendants, 0);
    assert_eq!(p.ancestors, 0);
}

#[test]
fn pruning_from_args_preserves_root_query_order_and_raw_tokens() {
    let args = parse_with(&["uns", "-r", "1,foo", "-r", "2-3", "x.ts"]);
    let p = pruning_from_args(&args).expect("pruning options");
    let raws: Vec<&str> = p
        .roots
        .iter()
        .map(|q| match q {
            unsnarl_root_query::ParsedRootQuery::Line { raw, .. }
            | unsnarl_root_query::ParsedRootQuery::LineName { raw, .. }
            | unsnarl_root_query::ParsedRootQuery::Range { raw, .. }
            | unsnarl_root_query::ParsedRootQuery::RangeName { raw, .. }
            | unsnarl_root_query::ParsedRootQuery::Name { raw, .. }
            | unsnarl_root_query::ParsedRootQuery::LineOrName { raw, .. } => raw.as_str(),
        })
        .collect();
    assert_eq!(raws, vec!["1", "foo", "2-3"]);
}

// `depths_from_args` mirrors `resolveDepths` in
// `ts/src/cli/run-cli/normalize-cli-options.ts`. The cases below
// cover the same precedence rules (--depth seeds both axes, then
// --depth-function / --depth-block override their halves) plus the
// no-flag default.

#[test]
fn depths_from_args_no_flag_seeds_default_depth_across_every_kind() {
    let args = parse_with(&["uns", "x.ts"]);
    let d = depths_from_args(&args);
    assert_eq!(d.function, DEFAULT_DEPTH);
    assert_eq!(d.r#if, DEFAULT_DEPTH);
    assert_eq!(d.r#for, DEFAULT_DEPTH);
    assert_eq!(d.r#while, DEFAULT_DEPTH);
    assert_eq!(d.switch, DEFAULT_DEPTH);
    assert_eq!(d.try_catch_finally, DEFAULT_DEPTH);
    assert_eq!(d.block, DEFAULT_DEPTH);
}

#[test]
fn depths_from_args_dash_depth_seeds_both_axes() {
    let args = parse_with(&["uns", "--depth", "3", "x.ts"]);
    let d = depths_from_args(&args);
    assert_eq!(d.function, NestingDepth(3));
    assert_eq!(d.r#if, NestingDepth(3));
    assert_eq!(d.block, NestingDepth(3));
}

#[test]
fn depths_from_args_depth_function_overrides_only_function() {
    let args = parse_with(&["uns", "--depth", "5", "--depth-function", "1", "x.ts"]);
    let d = depths_from_args(&args);
    assert_eq!(d.function, NestingDepth(1));
    assert_eq!(d.r#if, NestingDepth(5));
    assert_eq!(d.block, NestingDepth(5));
}

#[test]
fn depths_from_args_depth_block_overrides_only_block_kinds() {
    let args = parse_with(&["uns", "--depth", "5", "--depth-block", "1", "x.ts"]);
    let d = depths_from_args(&args);
    assert_eq!(d.function, NestingDepth(5));
    assert_eq!(d.r#if, NestingDepth(1));
    assert_eq!(d.r#for, NestingDepth(1));
    assert_eq!(d.r#while, NestingDepth(1));
    assert_eq!(d.switch, NestingDepth(1));
    assert_eq!(d.try_catch_finally, NestingDepth(1));
    assert_eq!(d.block, NestingDepth(1));
}

#[test]
fn depths_from_args_depth_function_only_keeps_block_at_default() {
    let args = parse_with(&["uns", "--depth-function", "2", "x.ts"]);
    let d = depths_from_args(&args);
    assert_eq!(d.function, NestingDepth(2));
    assert_eq!(d.r#if, DEFAULT_DEPTH);
    assert_eq!(d.block, DEFAULT_DEPTH);
}

#[test]
fn depths_from_args_depth_block_only_keeps_function_at_default() {
    let args = parse_with(&["uns", "--depth-block", "2", "x.ts"]);
    let d = depths_from_args(&args);
    assert_eq!(d.function, DEFAULT_DEPTH);
    assert_eq!(d.r#if, NestingDepth(2));
    assert_eq!(d.block, NestingDepth(2));
}

#[test]
fn depths_from_args_depth_zero_is_explicit_not_falsy() {
    let args = parse_with(&["uns", "--depth", "0", "x.ts"]);
    let d = depths_from_args(&args);
    assert_eq!(d.function, NestingDepth(0));
    assert_eq!(d.block, NestingDepth(0));
}

#[test]
fn run_to_with_dash_depth_collapses_deep_function_in_markdown_query_block() {
    // End-to-end: parse argv with `--depth 1`, dispatch through
    // `run_to` against a tempfile that contains a function body, and
    // verify the markdown emitter rendered the depth flag in the
    // `## Query` section. This is the seam that bench-parity does
    // not exercise (it never passes `--depth*`).
    use std::io::Write;
    let mut tmp = tempfile::Builder::new()
        .suffix(".ts")
        .tempfile()
        .expect("create tempfile");
    writeln!(tmp, "function outer() {{\n  function inner() {{}}\n}}").expect("write tempfile");
    let path = tmp
        .path()
        .to_str()
        .expect("tempfile path utf-8")
        .to_string();
    let out = capture_stdout(&["uns", "-f", "markdown", "--depth", "1", &path]);
    // Markdown emitter renders the chosen depth via formatDepthQuery
    // in the `## Query` block when at least one kind diverges from
    // DEFAULT_DEPTH.
    assert!(
        out.contains("--depth 1"),
        "expected --depth 1 in Query block, got:\n{out}"
    );
}

#[test]
fn run_to_with_dash_r_emits_pruning_summary_in_mermaid_output() {
    // End-to-end: parse argv with `-r`, dispatch through `run_to`,
    // verify the mermaid emitter rendered the pruning summary comment
    // the CLI is wired to produce. This is the seam that bench-parity
    // does not exercise (it never passes `-r`).
    use std::io::Write;
    let mut tmp = tempfile::Builder::new()
        .suffix(".ts")
        .tempfile()
        .expect("create tempfile");
    writeln!(tmp, "const a = 1;\nconst b = a;\nconst c = b;").expect("write tempfile");
    let path = tmp
        .path()
        .to_str()
        .expect("tempfile path utf-8")
        .to_string();
    let out = capture_stdout(&["uns", "-f", "mermaid", "-r", "a", "-C", "1", &path]);
    // The mermaid emitter prefixes the diagram with a `%% pruning
    // roots <summary> ancestors=<N> descendants=<M>` comment when
    // `VisualGraph.pruning` is populated. Confirming the summary line
    // is enough to prove that prune-from-CLI plumbing fired and the
    // pruned graph reached the emitter.
    assert!(
        out.contains("%% pruning roots a=1 ancestors=1 descendants=1"),
        "expected pruning summary line, got:\n{out}"
    );
}
