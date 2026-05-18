//! CLI argument parsing tests for the Step 2 `clap` skeleton.
//!
//! These cover the behavioral surface that the Step 2 stub guarantees:
//! every flag in the TS commander definition is accepted, defaults
//! match, the `-H` optional-value semantics line up with commander's
//! `[queries]` form, and `-v` / `-h` resolve through clap's standard
//! display actions (exit code is exercised here via `ErrorKind`; the
//! actual stderr writing is exercised by the binary in `main.rs`).
//!
//! Per-flag validation (`ValueEnum` for enums, numeric range, query
//! grammar, mutual exclusion, plugin name lookup) is deferred to
//! Steps 3 / 4 / 5 per #108 and #111's discussion comments.

use clap::error::ErrorKind;
use clap::Parser;
use unsnarl::cli::args::Args;

fn parse(argv: &[&str]) -> Args {
    Args::try_parse_from(argv).expect("argv should parse")
}

#[test]
fn defaults_match_ts_commander() {
    let args = parse(&["uns", "input.ts"]);
    assert_eq!(args.file.as_deref(), Some("input.ts"));
    assert_eq!(args.format, "mermaid");
    assert!(args.pretty_json);
    assert!(args.mermaid_renderer.is_none());
    assert_eq!(args.color_theme, "dark");
    assert!(!args.stdin);
    assert_eq!(args.stdin_lang, "ts");
    assert!(args.roots.is_empty());
    assert!(args.highlight.is_none());
    assert!(args.descendants.is_none());
    assert!(args.ancestors.is_none());
    assert!(args.context.is_none());
    assert!(args.depth.is_none());
    assert!(args.depth_function.is_none());
    assert!(args.depth_block.is_none());
    assert!(args.out_dir.is_none());
    assert!(args.out_file.is_none());
    assert!(!args.debug);
    assert!(args.plugins.is_empty());
}

#[test]
fn file_argument_is_optional() {
    let args = parse(&["uns"]);
    assert!(args.file.is_none());
}

#[test]
fn format_flag_short_and_long() {
    assert_eq!(parse(&["uns", "-f", "json", "x.ts"]).format, "json");
    assert_eq!(parse(&["uns", "--format", "ir", "x.ts"]).format, "ir");
}

#[test]
fn no_pretty_json_sets_pretty_json_false() {
    let args = parse(&["uns", "--no-pretty-json", "x.ts"]);
    assert!(!args.pretty_json);
}

#[test]
fn mermaid_renderer_is_raw_string_in_step2() {
    // Step 3 will validate against {elk, dagre}; Step 2 accepts anything.
    let args = parse(&["uns", "--mermaid-renderer", "dagre", "x.ts"]);
    assert_eq!(args.mermaid_renderer.as_deref(), Some("dagre"));
    let args = parse(&["uns", "--mermaid-renderer", "unknown-engine", "x.ts"]);
    assert_eq!(args.mermaid_renderer.as_deref(), Some("unknown-engine"));
}

#[test]
fn color_theme_long_only() {
    let args = parse(&["uns", "--color-theme", "light", "x.ts"]);
    assert_eq!(args.color_theme, "light");
}

#[test]
fn stdin_flags() {
    let args = parse(&["uns", "--stdin", "--stdin-lang", "tsx"]);
    assert!(args.stdin);
    assert_eq!(args.stdin_lang, "tsx");
}

#[test]
fn roots_repeatable_keeps_comma_strings_unsplit() {
    // Comma splitting happens in the query parser at Step 5. In Step 2
    // each occurrence is preserved as a raw string.
    let args = parse(&["uns", "-r", "1", "-r", "2,3", "x.ts"]);
    assert_eq!(args.roots, vec!["1".to_string(), "2,3".to_string()]);
}

#[test]
fn highlight_absent_is_none() {
    let args = parse(&["uns", "x.ts"]);
    assert!(args.highlight.is_none());
}

#[test]
fn highlight_present_without_value_at_end() {
    // `x.ts -H` -> positional consumed first, then -H with no value.
    let args = parse(&["uns", "x.ts", "-H"]);
    assert_eq!(args.file.as_deref(), Some("x.ts"));
    assert_eq!(args.highlight, Some(None));
}

#[test]
fn highlight_with_inline_value_via_equals() {
    let args = parse(&["uns", "--highlight=foo", "x.ts"]);
    assert_eq!(args.highlight, Some(Some("foo".to_string())));
    assert_eq!(args.file.as_deref(), Some("x.ts"));
}

#[test]
fn highlight_short_consumes_next_token_as_value() {
    // `-H foo.ts` matches commander's `[queries]` form: foo.ts is the
    // highlight value (greedy), and there is no positional file.
    let args = parse(&["uns", "-H", "foo.ts"]);
    assert_eq!(args.highlight, Some(Some("foo.ts".to_string())));
    assert!(args.file.is_none());
}

#[test]
fn generation_flags_are_raw_strings_in_step2() {
    // Step 3 converts these to u32 and rejects negatives / non-numerics.
    let args = parse(&["uns", "-A", "2", "-B", "3", "-C", "4", "x.ts"]);
    assert_eq!(args.descendants.as_deref(), Some("2"));
    assert_eq!(args.ancestors.as_deref(), Some("3"));
    assert_eq!(args.context.as_deref(), Some("4"));
    // Non-numeric is accepted by Step 2 (deferred validation):
    let args = parse(&["uns", "-A", "abc", "x.ts"]);
    assert_eq!(args.descendants.as_deref(), Some("abc"));
}

#[test]
fn depth_flags_are_raw_strings_in_step2() {
    let args = parse(&[
        "uns",
        "--depth",
        "5",
        "--depth-function",
        "8",
        "--depth-block",
        "3",
        "x.ts",
    ]);
    assert_eq!(args.depth.as_deref(), Some("5"));
    assert_eq!(args.depth_function.as_deref(), Some("8"));
    assert_eq!(args.depth_block.as_deref(), Some("3"));
}

#[test]
fn out_dir_and_out_file_both_accepted_without_mutex() {
    // Mutual exclusion is Step 4.
    let args = parse(&[
        "uns",
        "-o",
        "build/",
        "--out-file",
        "build/graph.mmd",
        "x.ts",
    ]);
    assert_eq!(args.out_dir.as_deref(), Some("build/"));
    assert_eq!(args.out_file.as_deref(), Some("build/graph.mmd"));
}

#[test]
fn plugin_repeats_and_keeps_commas() {
    // Comma-splitting and bundled name validation are Step 4.
    let args = parse(&["uns", "--plugin", "react", "--plugin", "a,b", "x.ts"]);
    assert_eq!(args.plugins, vec!["react".to_string(), "a,b".to_string()]);
}

#[test]
fn debug_flag() {
    let args = parse(&["uns", "--debug", "x.ts"]);
    assert!(args.debug);
}

#[test]
fn unknown_flag_yields_exit_code_2() {
    let err = Args::try_parse_from(["uns", "--unknown"]).unwrap_err();
    assert_eq!(err.exit_code(), 2);
}

#[test]
fn version_short_lowercase_uses_display_version() {
    // TS uses `-v, --version`; Step 2 wires this through
    // ArgAction::Version so the exit code is 0 and the kind is
    // DisplayVersion (clap's standard handling).
    let err = Args::try_parse_from(["uns", "-v"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.exit_code(), 0);
}

#[test]
fn version_long_uses_display_version() {
    let err = Args::try_parse_from(["uns", "--version"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayVersion);
    assert_eq!(err.exit_code(), 0);
}

#[test]
fn help_short_and_long_use_display_help() {
    let err = Args::try_parse_from(["uns", "-h"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);
    assert_eq!(err.exit_code(), 0);
    let err = Args::try_parse_from(["uns", "--help"]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::DisplayHelp);
    assert_eq!(err.exit_code(), 0);
}

#[test]
fn highlight_serializes_as_false_when_absent() {
    let args = parse(&["uns", "x.ts"]);
    let v = serde_json::to_value(&args).unwrap();
    assert_eq!(v["highlight"], serde_json::Value::Bool(false));
}

#[test]
fn highlight_serializes_as_true_when_no_inline_value() {
    let args = parse(&["uns", "x.ts", "-H"]);
    let v = serde_json::to_value(&args).unwrap();
    assert_eq!(v["highlight"], serde_json::Value::Bool(true));
}

#[test]
fn highlight_serializes_as_string_when_inline_value_given() {
    let args = parse(&["uns", "--highlight=foo,bar"]);
    let v = serde_json::to_value(&args).unwrap();
    assert_eq!(
        v["highlight"],
        serde_json::Value::String("foo,bar".to_string())
    );
}

#[test]
fn version_field_is_excluded_from_serialization() {
    // The internal `version: ()` field used to wire `-v` through
    // ArgAction::Version must not appear in debug JSON output.
    let args = parse(&["uns", "x.ts"]);
    let v = serde_json::to_value(&args).unwrap();
    assert!(v.get("version").is_none());
}

#[test]
fn parsed_args_match_ts_field_names_in_camel_case() {
    // Mirrors `ParsedCliOptions` in `ts/src/cli/args/parsed-cli-options.ts`.
    let args = parse(&["uns", "x.ts"]);
    let v = serde_json::to_value(&args).unwrap();
    let obj = v.as_object().unwrap();
    for key in [
        "file",
        "format",
        "prettyJson",
        "mermaidRenderer",
        "colorTheme",
        "stdin",
        "stdinLang",
        "roots",
        "highlight",
        "descendants",
        "ancestors",
        "context",
        "depth",
        "depthFunction",
        "depthBlock",
        "outDir",
        "outFile",
        "debug",
        "plugins",
    ] {
        assert!(obj.contains_key(key), "missing key: {key}");
    }
}
