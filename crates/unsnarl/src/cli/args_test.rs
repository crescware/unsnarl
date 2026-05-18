use super::*;
use clap::error::ErrorKind;
use unsnarl_root_query::ParsedRootQuery;

fn parse(argv: &[&str]) -> Args {
    Args::try_parse_from(argv).expect("argv should parse")
}

fn parse_err_exit_code(argv: &[&str]) -> i32 {
    Args::try_parse_from(argv).unwrap_err().exit_code()
}

#[test]
fn defaults_match_ts_commander() {
    let args = parse(&["uns", "input.ts"]);
    assert_eq!(args.file.as_deref(), Some("input.ts"));
    assert_eq!(args.format, CliFormat::Mermaid);
    assert!(args.pretty_json);
    assert!(args.mermaid_renderer.is_none());
    assert_eq!(args.color_theme, CliColorTheme::Dark);
    assert!(!args.stdin);
    assert_eq!(args.stdin_lang, CliLanguage::Ts);
    assert!(args.roots.is_empty());
    assert_eq!(args.highlight, Highlight::Absent);
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
    assert_eq!(
        parse(&["uns", "-f", "json", "x.ts"]).format,
        CliFormat::Json
    );
    assert_eq!(
        parse(&["uns", "--format", "ir", "x.ts"]).format,
        CliFormat::Ir
    );
}

#[test]
fn format_accepts_all_registry_values() {
    for (raw, expected) in [
        ("mermaid", CliFormat::Mermaid),
        ("ir", CliFormat::Ir),
        ("json", CliFormat::Json),
        ("markdown", CliFormat::Markdown),
        ("stats", CliFormat::Stats),
    ] {
        assert_eq!(parse(&["uns", "-f", raw, "x.ts"]).format, expected);
    }
}

#[test]
fn format_rejects_unknown_value_with_exit_2() {
    assert_eq!(parse_err_exit_code(&["uns", "-f", "bogus", "x.ts"]), 2);
}

#[test]
fn no_pretty_json_sets_pretty_json_false() {
    let args = parse(&["uns", "--no-pretty-json", "x.ts"]);
    assert!(!args.pretty_json);
}

#[test]
fn mermaid_renderer_accepts_dagre_and_elk() {
    assert_eq!(
        parse(&["uns", "--mermaid-renderer", "dagre", "x.ts"]).mermaid_renderer,
        Some(CliMermaidRenderer::Dagre)
    );
    assert_eq!(
        parse(&["uns", "--mermaid-renderer", "elk", "x.ts"]).mermaid_renderer,
        Some(CliMermaidRenderer::Elk)
    );
}

#[test]
fn mermaid_renderer_rejects_unknown_value_with_exit_2() {
    assert_eq!(
        parse_err_exit_code(&["uns", "--mermaid-renderer", "unknown-engine", "x.ts"]),
        2
    );
}

#[test]
fn color_theme_long_only() {
    let args = parse(&["uns", "--color-theme", "light", "x.ts"]);
    assert_eq!(args.color_theme, CliColorTheme::Light);
}

#[test]
fn color_theme_rejects_unknown_value_with_exit_2() {
    assert_eq!(
        parse_err_exit_code(&["uns", "--color-theme", "neon", "x.ts"]),
        2
    );
}

#[test]
fn stdin_flags() {
    let args = parse(&["uns", "--stdin", "--stdin-lang", "tsx"]);
    assert!(args.stdin);
    assert_eq!(args.stdin_lang, CliLanguage::Tsx);
}

#[test]
fn stdin_lang_accepts_all_languages() {
    for (raw, expected) in [
        ("ts", CliLanguage::Ts),
        ("tsx", CliLanguage::Tsx),
        ("js", CliLanguage::Js),
        ("jsx", CliLanguage::Jsx),
    ] {
        assert_eq!(
            parse(&["uns", "--stdin", "--stdin-lang", raw]).stdin_lang,
            expected
        );
    }
}

#[test]
fn stdin_lang_rejects_unknown_value_with_exit_2() {
    assert_eq!(
        parse_err_exit_code(&["uns", "--stdin", "--stdin-lang", "coffee"]),
        2
    );
}

#[test]
fn roots_repeatable_flattens_comma_lists_into_parsed_queries() {
    let args = parse(&["uns", "-r", "1", "-r", "2,3", "x.ts"]);
    assert_eq!(
        args.roots,
        vec![
            ParsedRootQuery::Line {
                line: 1,
                raw: "1".to_string(),
            },
            ParsedRootQuery::Line {
                line: 2,
                raw: "2".to_string(),
            },
            ParsedRootQuery::Line {
                line: 3,
                raw: "3".to_string(),
            },
        ],
    );
}

#[test]
fn roots_invalid_value_yields_exit_2() {
    assert_eq!(parse_err_exit_code(&["uns", "-r", "foo-bar", "x.ts"]), 2);
}

#[test]
fn highlight_absent_is_absent_variant() {
    let args = parse(&["uns", "x.ts"]);
    assert_eq!(args.highlight, Highlight::Absent);
}

#[test]
fn highlight_present_without_value_at_end() {
    let args = parse(&["uns", "x.ts", "-H"]);
    assert_eq!(args.file.as_deref(), Some("x.ts"));
    assert_eq!(args.highlight, Highlight::NoValue);
}

#[test]
fn highlight_with_inline_value_via_equals() {
    let args = parse(&["uns", "--highlight=foo", "x.ts"]);
    assert_eq!(
        args.highlight,
        Highlight::Value(vec![ParsedRootQuery::Name {
            name: "foo".to_string(),
            raw: "foo".to_string(),
        }]),
    );
    assert_eq!(args.file.as_deref(), Some("x.ts"));
}

#[test]
fn highlight_short_consumes_next_token_as_value() {
    let args = parse(&["uns", "-H", "foo"]);
    assert_eq!(
        args.highlight,
        Highlight::Value(vec![ParsedRootQuery::Name {
            name: "foo".to_string(),
            raw: "foo".to_string(),
        }]),
    );
    assert!(args.file.is_none());
}

#[test]
fn highlight_invalid_inline_value_yields_exit_2() {
    assert_eq!(
        parse_err_exit_code(&["uns", "--highlight=foo-bar", "x.ts"]),
        2
    );
}

#[test]
fn generation_flags_parse_non_negative_integers() {
    let args = parse(&["uns", "-A", "2", "-B", "3", "-C", "4", "x.ts"]);
    assert_eq!(args.descendants, Some(2));
    assert_eq!(args.ancestors, Some(3));
    assert_eq!(args.context, Some(4));
}

#[test]
fn generation_flags_accept_zero() {
    let args = parse(&["uns", "-A", "0", "x.ts"]);
    assert_eq!(args.descendants, Some(0));
}

#[test]
fn generation_flags_reject_negative_with_exit_2() {
    assert_eq!(parse_err_exit_code(&["uns", "-A", "-1", "x.ts"]), 2);
}

#[test]
fn generation_flags_reject_non_digit_string_with_exit_2() {
    assert_eq!(parse_err_exit_code(&["uns", "-A", "abc", "x.ts"]), 2);
}

#[test]
fn generation_flags_reject_decimal_with_exit_2() {
    assert_eq!(parse_err_exit_code(&["uns", "-B", "1.5", "x.ts"]), 2);
}

#[test]
fn generation_flags_reject_leading_plus_with_exit_2() {
    assert_eq!(parse_err_exit_code(&["uns", "-C", "+1", "x.ts"]), 2);
}

#[test]
fn depth_flags_parse_non_negative_integers() {
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
    assert_eq!(args.depth, Some(5));
    assert_eq!(args.depth_function, Some(8));
    assert_eq!(args.depth_block, Some(3));
}

#[test]
fn depth_flags_reject_negative_with_exit_2() {
    assert_eq!(parse_err_exit_code(&["uns", "--depth", "-1", "x.ts"]), 2);
}

#[test]
fn depth_flags_reject_non_digit_string_with_exit_2() {
    assert_eq!(
        parse_err_exit_code(&["uns", "--depth-function", "abc", "x.ts"]),
        2
    );
}

#[test]
fn depth_flags_reject_decimal_with_exit_2() {
    assert_eq!(
        parse_err_exit_code(&["uns", "--depth-block", "2.5", "x.ts"]),
        2
    );
}

#[test]
fn out_dir_alone_accepted() {
    let args = parse(&["uns", "-o", "build/", "x.ts"]);
    assert_eq!(args.out_dir.as_deref(), Some("build/"));
    assert!(args.out_file.is_none());
}

#[test]
fn out_file_alone_accepted() {
    let args = parse(&["uns", "--out-file", "build/graph.mmd", "x.ts"]);
    assert!(args.out_dir.is_none());
    assert_eq!(args.out_file.as_deref(), Some("build/graph.mmd"));
}

#[test]
fn out_dir_and_out_file_conflict_with_exit_2() {
    assert_eq!(
        parse_err_exit_code(&[
            "uns",
            "-o",
            "build/",
            "--out-file",
            "build/graph.mmd",
            "x.ts",
        ]),
        2
    );
}

#[test]
fn derived_basename_is_none_when_out_dir_is_absent() {
    assert!(parse(&["uns", "x.ts"]).derived_basename.is_none());
    assert!(parse(&["uns", "--out-file", "build/graph.mmd", "x.ts"])
        .derived_basename
        .is_none());
}

#[test]
fn derived_basename_from_positional_file_strips_extension() {
    let args = parse(&["uns", "-o", "build", "src/deep/foo.ts"]);
    assert_eq!(args.derived_basename.as_deref(), Some("foo"));
}

#[test]
fn derived_basename_uses_root_token_over_input_file() {
    let args = parse(&["uns", "-r", "render", "-o", "build", "x.ts"]);
    assert_eq!(args.derived_basename.as_deref(), Some("render"));
}

#[test]
fn derived_basename_joins_multiple_roots_with_plus() {
    let args = parse(&["uns", "-r", "render,foo", "-o", "build", "x.ts"]);
    assert_eq!(args.derived_basename.as_deref(), Some("render+foo"));
}

#[test]
fn derived_basename_appends_radius_suffix_in_a_b_c_order() {
    let args = parse(&[
        "uns", "-r", "render", "-A", "1", "-B", "2", "-o", "build", "x.ts",
    ]);
    assert_eq!(args.derived_basename.as_deref(), Some("render-a1-b2"));
}

#[test]
fn derived_basename_drops_c_when_both_a_and_b_are_explicit() {
    let args = parse(&[
        "uns", "-r", "render", "-A", "1", "-B", "2", "-C", "3", "-o", "build", "x.ts",
    ]);
    assert_eq!(args.derived_basename.as_deref(), Some("render-a1-b2"));
}

#[test]
fn derived_basename_normalizes_line_or_name_to_l_n() {
    let args = parse(&["uns", "-r", "L12", "-o", "build", "x.ts"]);
    assert_eq!(args.derived_basename.as_deref(), Some("l12"));
    let args = parse(&["uns", "-r", "12", "-o", "build", "x.ts"]);
    assert_eq!(args.derived_basename.as_deref(), Some("l12"));
}

#[test]
fn stdin_with_out_dir_requires_roots_else_exit_2() {
    assert_eq!(parse_err_exit_code(&["uns", "--stdin", "-o", "build"]), 2);
}

#[test]
fn stdin_with_out_dir_and_roots_is_accepted_and_derives_root_basename() {
    let args = parse(&["uns", "--stdin", "-r", "render", "-o", "build"]);
    assert_eq!(args.derived_basename.as_deref(), Some("render"));
}

#[test]
fn stdin_with_out_file_is_accepted_without_roots() {
    let args = parse(&["uns", "--stdin", "--out-file", "build/graph.mmd"]);
    assert!(args.derived_basename.is_none());
    assert_eq!(args.out_file.as_deref(), Some("build/graph.mmd"));
}

#[test]
fn derived_basename_serializes_as_camel_case_field_under_out_dir() {
    let args = parse(&["uns", "-r", "render", "-o", "build", "x.ts"]);
    let v = serde_json::to_value(&args).unwrap();
    assert_eq!(
        v["derivedBasename"],
        serde_json::Value::String("render".into())
    );
}

#[test]
fn derived_basename_serializes_as_null_when_absent() {
    let args = parse(&["uns", "x.ts"]);
    let v = serde_json::to_value(&args).unwrap();
    assert_eq!(v["derivedBasename"], serde_json::Value::Null);
}

#[test]
fn plugin_short_name_accepted() {
    let args = parse(&["uns", "--plugin", "react", "x.ts"]);
    assert_eq!(args.plugins, vec!["react".to_string()]);
}

#[test]
fn plugin_prefixed_form_normalized_to_short() {
    let args = parse(&["uns", "--plugin", "unsnarl-plugin-react", "x.ts"]);
    assert_eq!(args.plugins, vec!["react".to_string()]);
}

#[test]
fn plugin_comma_list_splits_and_dedups() {
    let args = parse(&["uns", "--plugin", "react,unsnarl-plugin-react", "x.ts"]);
    assert_eq!(args.plugins, vec!["react".to_string()]);
}

#[test]
fn plugin_repeated_occurrences_dedupe_across_invocations() {
    let args = parse(&[
        "uns",
        "--plugin",
        "react",
        "--plugin",
        "unsnarl-plugin-react",
        "x.ts",
    ]);
    assert_eq!(args.plugins, vec!["react".to_string()]);
}

#[test]
fn plugin_empty_fragments_dropped() {
    let args = parse(&["uns", "--plugin", "react,,unsnarl-plugin-react", "x.ts"]);
    assert_eq!(args.plugins, vec!["react".to_string()]);
}

#[test]
fn plugin_unknown_name_yields_exit_2() {
    assert_eq!(parse_err_exit_code(&["uns", "--plugin", "vue", "x.ts"]), 2);
}

#[test]
fn plugin_unknown_in_comma_list_yields_exit_2() {
    assert_eq!(
        parse_err_exit_code(&["uns", "--plugin", "react,vue", "x.ts"]),
        2
    );
}

#[test]
fn plugin_unknown_across_repeated_occurrences_yields_exit_2() {
    assert_eq!(
        parse_err_exit_code(&["uns", "--plugin", "react", "--plugin", "vue", "x.ts"]),
        2
    );
}

#[test]
fn plugin_serializes_as_flat_string_array() {
    let args = parse(&[
        "uns",
        "--plugin",
        "react",
        "--plugin",
        "unsnarl-plugin-react",
        "x.ts",
    ]);
    let v = serde_json::to_value(&args).unwrap();
    assert_eq!(v["plugins"], serde_json::json!(["react"]));
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
fn highlight_serializes_as_parsed_root_query_array_when_inline_value_given() {
    let args = parse(&["uns", "--highlight=foo,bar"]);
    let v = serde_json::to_value(&args).unwrap();
    assert_eq!(
        v["highlight"],
        serde_json::json!([
            { "kind": "name", "name": "foo", "raw": "foo" },
            { "kind": "name", "name": "bar", "raw": "bar" },
        ]),
    );
}

#[test]
fn enum_fields_serialize_as_lowercase_strings() {
    let args = parse(&[
        "uns",
        "--mermaid-renderer",
        "dagre",
        "--stdin-lang",
        "tsx",
        "--color-theme",
        "light",
        "--stdin",
    ]);
    let v = serde_json::to_value(&args).unwrap();
    assert_eq!(v["format"], serde_json::Value::String("mermaid".into()));
    assert_eq!(v["stdinLang"], serde_json::Value::String("tsx".into()));
    assert_eq!(
        v["mermaidRenderer"],
        serde_json::Value::String("dagre".into())
    );
    assert_eq!(v["colorTheme"], serde_json::Value::String("light".into()));
}

#[test]
fn mermaid_renderer_serializes_as_null_when_default() {
    let args = parse(&["uns", "x.ts"]);
    let v = serde_json::to_value(&args).unwrap();
    assert_eq!(v["mermaidRenderer"], serde_json::Value::Null);
}

#[test]
fn version_field_is_excluded_from_serialization() {
    let args = parse(&["uns", "x.ts"]);
    let v = serde_json::to_value(&args).unwrap();
    assert!(v.get("version").is_none());
}

#[test]
fn parsed_args_match_ts_field_names_in_camel_case() {
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
