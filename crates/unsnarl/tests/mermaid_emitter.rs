//! End-to-end mermaid emitter test driving the full parse → analyse
//! → serialise → emit pipeline. The `unsnarl-emitter-mermaid` crate
//! itself does not depend on parser or analyser, so the integration
//! test lives here in `crates/unsnarl/tests/` (same pattern as
//! `flat_serializer.rs`), reusing `emit_mermaid_text` for the public
//! driver entry point.

use regex::Regex;
use unsnarl::pipeline::{emit_mermaid_text, PipelineRunOptions};
use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::{ColorTheme, DARK_THEME, LIGHT_THEME};
use unsnarl_emitter_mermaid::MermaidEmitter;
use unsnarl_ir::Language;

fn emit_with(code: &str, theme: &'static ColorTheme, language: Language) -> String {
    let ext = match language {
        Language::Ts => "ts",
        Language::Tsx => "tsx",
        Language::Js => "js",
        Language::Jsx => "jsx",
    };
    let source_path = format!("input.{ext}");
    emit_mermaid_text(
        code,
        &source_path,
        language,
        MermaidStrategy::Elk,
        theme,
        false,
        PipelineRunOptions::default(),
    )
    .expect("pipeline must succeed on the test inputs")
}

fn emit(code: &str) -> String {
    emit_with(code, &DARK_THEME, Language::Ts)
}

fn emit_lang(code: &str, language: Language) -> String {
    emit_with(code, &DARK_THEME, language)
}

fn lines(out: &str) -> Vec<&str> {
    out.split('\n').collect()
}

fn edges_for(out: &str) -> Vec<&str> {
    lines(out)
        .into_iter()
        .filter(|v| v.contains(" -->|"))
        .collect()
}

fn count_matches(out: &str, re_src: &str) -> usize {
    let re = Regex::new(re_src).expect("test regex");
    lines(out)
        .into_iter()
        .filter(|line| re.is_match(line))
        .count()
}

fn matches(out: &str, re_src: &str) -> bool {
    Regex::new(re_src).expect("test regex").is_match(out)
}

fn node_id_of(out: &str, name: &str) -> String {
    let re = Regex::new(&format!(r#"(n_scope_0_{name}_\d+)\["[^"]*{name}[^"]*"\]"#))
        .expect("node_id_of regex");
    let caps = re
        .captures(out)
        .unwrap_or_else(|| panic!("node for {name:?} not found in:\n{out}"));
    caps.get(1)
        .expect("regex pattern guarantees capture group 1 when matched")
        .as_str()
        .to_string()
}

// ---- Top-level identification ---------------------------------------------

#[test]
fn identifies_as_mermaid() {
    assert_eq!(MermaidEmitter::FORMAT, "mermaid");
    assert_eq!(MermaidEmitter::CONTENT_TYPE, "text/vnd.mermaid");
}

#[test]
fn renderer_defaults_to_elk_and_prepends_an_init_directive() {
    let out = emit("const a = 1;\n");
    assert!(out.starts_with("%%{init: {\"flowchart\": {\"defaultRenderer\": \"elk\"}}}%%\n"));
}

#[test]
fn renderer_dagre_omits_the_init_directive_entirely() {
    let out = emit_mermaid_text(
        "const a = 1;\n",
        "input.ts",
        Language::Ts,
        MermaidStrategy::Dagre,
        &DARK_THEME,
        false,
        PipelineRunOptions::default(),
    )
    .expect("pipeline must succeed");
    assert!(!out.contains("%%{init"));
    assert!(matches(&out, r"^flowchart RL\n"));
}

#[test]
fn emits_flowchart_rl_with_one_node_per_declared_variable() {
    let out = emit("const a = 1;\nconst b = a;\n");
    assert!(matches(&out, r#"^%%\{init:.*"elk".*\}%%\nflowchart RL\n"#));
    assert!(out.contains("\"a<br/>L1\""));
    assert!(out.contains("\"unused b<br/>L2\""));
}

#[test]
fn decorates_labels_per_definition_kind() {
    let out = emit(
        &[
            "import imp from 'x';",
            "function foo() { return 1; }",
            "class Bar {}",
            "function take(p: number) { try { p; } catch (e) { e; } }",
            "const used = imp;",
            "const a = take;",
            "const b = foo;",
            "const c = Bar;",
            "const d = used;",
            "void a; void b; void c; void d;",
        ]
        .join("\n"),
    );
    assert!(out.contains("\"import imp<br/>"));
    assert!(out.contains("\"foo()<br/>"));
    assert!(out.contains("\"class Bar<br/>"));
    assert!(out.contains("\"take()<br/>"));
    assert!(out.contains("\"p<br/>"));
    assert!(!out.contains("\"param p<br/>"));
    assert!(out.contains("\"catch e<br/>"));
    assert!(out.contains("\"used<br/>"));
    assert!(!matches(&out, r#"" : Variable"#));
}

#[test]
fn draws_a_data_flow_edge_from_the_source_variable_to_the_initialized_variable() {
    let out = emit("const a = 1;\nconst b = a;\n");
    assert!(matches(&out, r"n_scope_0_a_6 -->\|read\| n_scope_0_b_19"));
}

#[test]
fn attaches_read_call_edge_from_callee_to_the_variable_receiving_the_call_result() {
    let out = emit("function f() {}\nconst x = f();\n");
    assert!(matches(
        &out,
        r"n_scope_0_f_9 -->\|read,call\| n_scope_0_x_"
    ));
}

#[test]
fn renders_a_function_as_a_subgraph_and_routes_return_through_a_return_subgraph_with_per_use_nodes()
{
    let out = emit("function f() {\n  const x = 1;\n  return x;\n}\n");
    assert!(matches(&out, r#"subgraph s_scope_\d+\["f\(\)"#));
    assert!(matches(&out, r#"n_scope_0_f_9\["unused f\(\)<br/>L1"\]"#));
    assert!(out.contains("direction RL"));
    assert!(matches(
        &out,
        r#"subgraph s_return_scope_0_f_9_\w+\["return L3"\]"#
    ));
    assert!(matches(&out, r#"ret_use_\w+\["x<br/>L3"\]"#));
    assert!(matches(&out, r"n_scope_1_x_\d+ -->\|read\| ret_use_\w+"));
    assert!(out.contains("end"));
}

#[test]
fn subgraphs_arrow_functions_assigned_to_a_const() {
    let out = emit("const fn = (p: number) => p + 1;\n");
    assert!(matches(&out, r#"subgraph s_scope_\d+\["fn\(\)"#));
    assert!(matches(&out, r#"n_scope_0_fn_6\["unused fn\(\)<br/>L1"\]"#));
    assert!(matches(
        &out,
        r#"subgraph s_return_scope_0_fn_6_\w+\["return L1"\]"#
    ));
    assert!(matches(&out, r"n_scope_1_p_\d+ -->\|read\| ret_use_\w+"));
}

#[test]
fn subgraphs_function_expressions_assigned_to_a_const() {
    let out = emit("const fn = function inner(p: number) { return p; };\n");
    assert!(matches(&out, r#"subgraph s_scope_\d+\["fn\(\)"#));
    assert!(matches(&out, r#"n_scope_0_fn_6\["unused fn\(\)<br/>L1"\]"#));
    assert!(matches(
        &out,
        r#"subgraph s_return_scope_0_fn_6_\w+\["return L1"\]"#
    ));
}

#[test]
fn multi_line_jsx_opening_tags_render_as_escaped_name_with_open_close_range() {
    let code = [
        "import { A } from 'm';",
        "const App = () => (",
        "  <A>",
        "    hello",
        "  </A>",
        ");",
    ]
    .join("\n");
    let out = emit_lang(&code, Language::Tsx);
    assert!(matches(&out, r#"ret_use_\w+\["&lt;A&gt;<br/>L3-5"\]"#));
}

#[test]
fn single_line_jsx_elements_collapse_to_a_single_line_label() {
    let code = ["import { A } from 'm';", "const App = () => <A>hi</A>;"].join("\n");
    let out = emit_lang(&code, Language::Tsx);
    assert!(matches(&out, r#"ret_use_\w+\["&lt;A&gt;<br/>L2"\]"#));
    assert!(!matches(&out, r"L2-"));
}

#[test]
fn a_non_jsx_return_use_keeps_the_bare_name_without_angle_bracket_wrapping() {
    let out = emit("function f(a) { return a; }\n");
    assert!(matches(&out, r#"ret_use_\w+\["a<br/>L1"\]"#));
    assert!(!out.contains("&lt;a&gt;"));
}

#[test]
fn a_multi_line_return_statement_yields_a_return_subgraph_spanning_the_whole_statement() {
    let code = [
        "function build() {",
        "  const a = 1;",
        "  const b = 2;",
        "  return {",
        "    a,",
        "    b,",
        "  };",
        "}",
    ]
    .join("\n");
    let out = emit(&code);
    assert!(matches(
        &out,
        r#"subgraph s_scope_\d+\["build\(\)<br/>L1-8"\]"#
    ));
    assert!(matches(
        &out,
        r#"subgraph s_return_scope_\w+\["return L4-7"\]"#
    ));
    assert!(matches(&out, r#"ret_use_\w+\["a<br/>L5"\]"#));
    assert!(matches(&out, r#"ret_use_\w+\["b<br/>L6"\]"#));
}

#[test]
fn a_block_body_arrow_with_explicit_return_uses_the_return_statement_span() {
    let code = ["const fn = (x) => {", "  return x;", "};"].join("\n");
    let out = emit(&code);
    assert!(matches(
        &out,
        r#"subgraph s_scope_\d+\["fn\(\)<br/>L1-3"\]"#
    ));
    assert!(matches(
        &out,
        r#"subgraph s_return_scope_\w+\["return L2"\]"#
    ));
    assert!(matches(&out, r#"ret_use_\w+\["x<br/>L2"\]"#));
}

#[test]
fn a_multi_line_arrow_with_expression_body_uses_the_body_span_as_the_implicit_return() {
    let code = ["const fn = (x) => (", "  x + 1", ");"].join("\n");
    let out = emit(&code);
    assert!(matches(
        &out,
        r#"subgraph s_scope_\d+\["fn\(\)<br/>L1-3"\]"#
    ));
    assert!(matches(
        &out,
        r#"subgraph s_return_scope_\w+\["return L1-3"\]"#
    ));
    assert!(matches(&out, r#"ret_use_\w+\["x<br/>L2"\]"#));
}

#[test]
fn each_return_statement_renders_its_own_subgraph_they_are_not_merged() {
    let code = [
        "function pick(k) {",
        "  const a = 1;",
        "  const b = 2;",
        "  if (k) {",
        "    return a;",
        "  }",
        "  return b;",
        "}",
    ]
    .join("\n");
    let out = emit(&code);
    assert_eq!(
        count_matches(&out, r#"^\s*subgraph s_return_scope_\w+\["return L5"\]"#),
        1
    );
    assert_eq!(
        count_matches(&out, r#"^\s*subgraph s_return_scope_\w+\["return L7"\]"#),
        1
    );
    assert!(!matches(&out, r"return L5-7"));
}

#[test]
fn marks_unused_declarations_with_an_unused_prefix_in_the_label() {
    let out = emit("const a = 1;\nconst unused = 2;\nconst b = a;\n");
    assert!(!out.contains("classDef unused"));
    assert!(!matches(&out, r"(?m)^\s*class n_scope_"));
    assert!(matches(
        &out,
        r#"n_scope_0_unused_\d+\["unused unused<br/>L2"\]"#
    ));
    assert!(matches(&out, r#"n_scope_0_b_\d+\["unused b<br/>L3"\]"#));
    assert!(matches(&out, r#"n_scope_0_a_\d+\["a<br/>L1"\]"#));
}

#[test]
fn renders_implicit_global_variable_as_a_global_node_when_used_directly() {
    let out = emit("function f() { return globalThing; }\n");
    assert!(out.contains("\"global globalThing\""));
    assert!(!out.contains("global globalThing<br/>"));
}

#[test]
fn renders_implicit_global_variable_that_only_appears_as_a_member_receiver() {
    let out = emit("const xs = Object.keys(arg);\n");
    assert!(out.contains("\"global Object\""));
    assert!(out.contains("\"global arg\""));
    assert!(!out.contains("global Object<br/>"));
    assert!(!out.contains("global arg<br/>"));
}

#[test]
fn a_top_level_call_expression_statement_renders_as_per_statement_node() {
    let out = emit("function f() {}\nf();\n");
    assert!(matches(&out, r#"expr_stmt_\d+\["f\(\)<br/>L2"\]"#));
    assert!(!out.contains("module_root((module))"));
}

#[test]
fn a_case_predicate_ref_still_falls_back_to_module_root() {
    let out = emit(
        &[
            "let label = '';",
            "const a = 1;",
            "switch (a) {",
            "  case Number.MAX_SAFE_INTEGER:",
            "    label = 'max';",
            "    break;",
            "  default:",
            "    label = 'other';",
            "}",
            "const r = label;",
        ]
        .join("\n"),
    );
    assert!(out.contains("module_root((module))"));
}

#[test]
fn subgraphs_try_catch_finally_blocks_with_line_numbers() {
    let code = [
        "let v = 0;",
        "try {",
        "  v = 1;",
        "} catch (err) {",
        "  v = 2;",
        "} finally {",
        "  v = 3;",
        "}",
    ]
    .join("\n");
    let out = emit(&code);
    assert!(matches(&out, r#"subgraph s_scope_\d+\["try L2-4"\]"#));
    assert!(matches(&out, r#"subgraph s_scope_\d+\["catch L4-6"\]"#));
    assert!(matches(&out, r#"subgraph s_scope_\d+\["finally L6-8"\]"#));
}

#[test]
fn subgraphs_if_else_blocks() {
    let code = "let x = 0;\nif (true) {\n  x = 1;\n} else {\n  x = 2;\n}\n";
    let out = emit(code);
    assert!(matches(&out, r#"subgraph s_scope_\d+\["if L2-4"\]"#));
    assert!(matches(&out, r#"subgraph s_scope_\d+\["else L4-6"\]"#));
}

#[test]
fn subgraphs_switch_statements() {
    let code =
        "let l = '';\nconst k = 'a';\nswitch (k) {\n  case 'a': l = 'A'; break;\n  default: l = '?';\n}\n";
    let out = emit(code);
    assert!(matches(&out, r#"subgraph s_scope_\d+\["switch L3-6"\]"#));
}

#[test]
fn encodes_double_quotes_in_labels_with_html_entity_never_with_backslash() {
    let code =
        "let l = \"\";\nconst k = \"a\";\nswitch (k) {\n  case \"x\": l = \"x\"; break;\n}\n";
    let out = emit(code);
    assert!(!out.contains("\\\""));
    assert!(matches(&out, r"case &quot;x&quot;"));
}

#[test]
fn preserves_single_quotes_in_case_labels_verbatim_without_html_entities() {
    let code = "let l = '';\nconst k = 'a';\nswitch (k) {\n  case 'x': l = 'x'; break;\n}\n";
    let out = emit(code);
    assert!(matches(&out, r"case 'x' L\d+"));
    assert!(!out.contains("&quot;"));
    assert!(!out.contains("&apos;"));
}

#[test]
fn case_label_quote_style_mirrors_the_source_verbatim_except_for_html_escaping() {
    let single =
        emit("let l = '';\nconst k = 'a';\nswitch (k) {\n  case 'x': l = 'x'; break;\n}\n");
    let double =
        emit("let l = \"\";\nconst k = \"a\";\nswitch (k) {\n  case \"x\": l = \"x\"; break;\n}\n");
    assert!(matches(&single, r"case 'x' L\d+"));
    assert!(matches(&double, r"case &quot;x&quot; L\d+"));
    assert!(!matches(&single, r"case &quot;"));
    assert!(!matches(&double, r"case '"));
}

#[test]
fn encodes_amp_lt_gt_in_case_labels_with_html_entities() {
    let code =
        "let l = 0;\nconst a = 1;\nconst b = 2;\nswitch (a) {\n  case (a & b): l = 1; break;\n  case (a < b ? 1 : 0): l = 2; break;\n}\n";
    let out = emit(code);
    assert!(!matches(&out, r#"case [^"]*&[^a-z]"#));
    assert!(out.contains("&amp;"));
    assert!(out.contains("&lt;"));
}

#[test]
fn expands_import_declarations_into_module_subgraphs() {
    let out = emit(
        &[
            "import def from 'some-default';",
            "import { named, other as renamed } from 'some-named';",
            "import * as ns from 'some-namespace';",
            "const a = def;",
            "const b = named;",
            "const c = renamed;",
            "const d = ns;",
        ]
        .join("\n"),
    );
    // Each import source becomes a module subgraph; the header names
    // the source and carries no line range.
    assert!(out.contains(r#"subgraph sg_some_default["module some-default"]"#));
    assert!(out.contains(r#"subgraph sg_some_named["module some-named"]"#));
    assert!(out.contains(r#"subgraph sg_some_namespace["module some-namespace"]"#));
    // The renamed import keeps its original-name intermediate, and the
    // intermediate -> local edge survives inside the subgraph.
    assert!(matches(
        &out,
        r#"import_some_named__other\["import other<br/>L2"\]"#
    ));
    assert!(matches(
        &out,
        r"import_some_named__other -->\|read\| n_scope_0_renamed_"
    ));
    // The old `mod_*` source nodes and their `module -> binding` edges
    // are gone -- containment replaces them.
    assert!(!out.contains("mod_some_default"));
    assert!(!out.contains("mod_some_named"));
    assert!(!out.contains("mod_some_namespace"));
    assert!(!matches(&out, r"mod_\w+ -->"));
    // Local binding labels are unchanged.
    assert!(out.contains("\"import ns<br/>"));
    assert!(out.contains("\"import def<br/>"));
    assert!(out.contains("\"import named<br/>"));
    assert!(out.contains("\"renamed<br/>"));
    assert!(!out.contains("\"import renamed<br/>"));
}

// ---- MermaidEmitter rendering: switch with break --------------------------

fn switch_with_break_code() -> String {
    [
        "let label = \"\";",
        "const kind = \"a\";",
        "switch (kind) {",
        "  case \"a\":",
        "    label = \"alpha\";",
        "    break;",
        "  case \"b\":",
        "    label = \"beta\";",
        "    break;",
        "  default:",
        "    label = \"other\";",
        "    break;",
        "}",
        "const result = label;",
    ]
    .join("\n")
}

#[test]
fn switch_with_break_each_case_becomes_its_own_labelled_subgraph() {
    let out = emit(&switch_with_break_code());
    assert_eq!(
        count_matches(
            &out,
            r#"^\s*subgraph s_scope_\d+\["case .* L\d+(?:-\d+)?"\]"#
        ),
        2
    );
    assert_eq!(
        count_matches(
            &out,
            r#"^\s*subgraph s_scope_\d+\["default L\d+(?:-\d+)?"\]"#
        ),
        1
    );
    assert!(!out.contains("|fallthrough|"));
}

#[test]
fn switch_with_break_declaration_fans_out_to_every_case_via_one_set_edge_each() {
    let out = emit(&switch_with_break_code());
    let decl = "n_scope_0_label_4";
    let set_edges: Vec<&str> = edges_for(&out)
        .into_iter()
        .filter(|v| v.starts_with(&format!("  {decl} -->")) && v.contains("|set|"))
        .collect();
    assert_eq!(set_edges.len(), 3);
}

#[test]
fn switch_with_break_each_case_fans_into_result_via_one_read_edge() {
    let out = emit(&switch_with_break_code());
    let result = node_id_of(&out, "result");
    let reads: Vec<&str> = edges_for(&out)
        .into_iter()
        .filter(|v| v.contains("|read|") && v.ends_with(&result))
        .collect();
    assert_eq!(reads.len(), 3);
}

#[test]
fn switch_with_break_discriminant_routes_into_the_switch_discriminant_anchor() {
    let out = emit(&switch_with_break_code());
    assert!(matches(
        &out,
        r"n_scope_0_kind_\d+ -->\|read\| switch_discriminant_scope_0_\d+"
    ));
    assert!(!matches(
        &out,
        r"n_scope_0_kind_\d+ -->\|read\| module_root"
    ));
    assert!(matches(
        &out,
        r#"(?m)^\s*switch_discriminant_scope_0_\d+\{"switch \(\)<br/>L\d+"\}"#
    ));
}

// ---- MermaidEmitter rendering: switch with fallthrough --------------------

fn switch_with_fallthrough_code() -> String {
    [
        "let label = \"\";",
        "const kind = \"a\";",
        "switch (kind) {",
        "  case \"a\":",
        "    label = \"alpha\";",
        "  case \"b\":",
        "    label = \"beta\";",
        "  default:",
        "    label = \"other\";",
        "}",
        "const result = label;",
    ]
    .join("\n")
}

#[test]
fn switch_with_fallthrough_declaration_only_emits_one_set_edge_into_the_head_case() {
    let out = emit(&switch_with_fallthrough_code());
    let set_edges: Vec<&str> = edges_for(&out)
        .into_iter()
        .filter(|v| v.starts_with("  n_scope_0_label_4 -->") && v.contains("|set|"))
        .collect();
    assert_eq!(set_edges.len(), 1);
}

#[test]
fn switch_with_fallthrough_non_terminal_cases_are_stitched_together_with_fallthrough_edges() {
    let out = emit(&switch_with_fallthrough_code());
    let ft: Vec<&str> = edges_for(&out)
        .into_iter()
        .filter(|v| v.contains("|fallthrough|"))
        .collect();
    assert_eq!(ft.len(), 2);
}

#[test]
fn switch_with_fallthrough_only_the_terminal_case_feeds_result() {
    let out = emit(&switch_with_fallthrough_code());
    let result = node_id_of(&out, "result");
    let reads: Vec<&str> = edges_for(&out)
        .into_iter()
        .filter(|v| v.contains("|read|") && v.ends_with(&result))
        .collect();
    assert_eq!(reads.len(), 1);
}

// ---- MermaidEmitter rendering: if/else ------------------------------------

fn if_else_code() -> String {
    [
        "let counter = 0;",
        "const flag = true;",
        "if (flag) {",
        "  counter = 1;",
        "} else {",
        "  counter = 2;",
        "}",
        "const result = counter;",
    ]
    .join("\n")
}

#[test]
fn if_else_outer_if_else_container_subgraph_wraps_both_arms() {
    let out = emit(&if_else_code());
    assert!(matches(
        &out,
        r#"(?m)^\s*subgraph cont_if_scope_0_\d+\["if-else L3-7"\]"#
    ));
    assert!(matches(
        &out,
        r#"(?m)^\s*subgraph s_scope_\d+\["if L3-5"\]"#
    ));
    assert!(matches(
        &out,
        r#"(?m)^\s*subgraph s_scope_\d+\["else L5-7"\]"#
    ));
}

#[test]
fn if_else_predicate_identifier_feeds_the_if_test_anchor_inside_consequent_subgraph() {
    let out = emit(&if_else_code());
    assert!(matches(
        &out,
        r"n_scope_0_flag_\d+ -->\|read\| if_test_scope_0_\d+"
    ));
    assert!(matches(
        &out,
        r#"(?m)^\s*if_test_scope_0_\d+\{"if \(\)<br/>L3"\}"#
    ));
}

#[test]
fn if_else_both_branches_independently_feed_result_the_declaration_does_not_bypass() {
    let out = emit(&if_else_code());
    let result = node_id_of(&out, "result");
    let reads: Vec<&str> = edges_for(&out)
        .into_iter()
        .filter(|v| v.contains("|read|") && v.ends_with(&result))
        .collect();
    assert_eq!(reads.len(), 2);
    let re_src = format!(r"n_scope_0_counter_4 -->\|read\| {result}\b");
    assert!(!matches(&out, &re_src));
}

// ---- MermaidEmitter rendering: if without else ----------------------------

fn if_without_else_code() -> String {
    [
        "let counter = 0;",
        "const flag = true;",
        "if (flag) {",
        "  counter = 1;",
        "}",
        "const result = counter;",
    ]
    .join("\n")
}

#[test]
fn if_without_else_there_is_no_if_else_container_just_a_bare_if_subgraph() {
    let out = emit(&if_without_else_code());
    assert!(!out.contains("if-else L"));
    assert!(matches(
        &out,
        r#"(?m)^\s*subgraph s_scope_\d+\["if L3-5"\]"#
    ));
}

#[test]
fn if_without_else_predicate_flows_into_the_if_test_anchor_inside_consequent_subgraph() {
    let out = emit(&if_without_else_code());
    assert!(matches(
        &out,
        r"n_scope_0_flag_\d+ -->\|read\| if_test_scope_0_\d+"
    ));
    assert!(matches(
        &out,
        r#"(?m)^\s*if_test_scope_0_\d+\{"if \(\)<br/>L3"\}"#
    ));
}

#[test]
fn if_without_else_result_has_two_origins_the_if_write_and_the_original_declaration() {
    let out = emit(&if_without_else_code());
    let result = node_id_of(&out, "result");
    let reads: Vec<&str> = edges_for(&out)
        .into_iter()
        .filter(|v| v.contains("|read|") && v.ends_with(&result))
        .collect();
    assert_eq!(reads.len(), 2);
    let re_src = format!(r"n_scope_0_counter_4 -->\|read\| {result}\b");
    assert!(matches(&out, &re_src));
}

// ---- catch parameter placement -------------------------------------------

#[test]
fn the_catch_parameter_node_lives_inside_the_catch_subgraph() {
    let code = [
        "let v = 0;",
        "try {",
        "  v = 1;",
        "} catch (err) {",
        "  v = 2;",
        "} finally {",
        "  v = 3;",
        "}",
    ]
    .join("\n");
    let out = emit(&code);
    let ls = lines(&out);
    let catch_start = ls
        .iter()
        .position(|v| v.contains("\"catch L4-6\""))
        .expect("catch subgraph header present");
    let catch_end_relative = ls[catch_start..]
        .iter()
        .position(|v| v.trim() == "end")
        .expect("catch subgraph close present");
    assert!(catch_end_relative > 0);
    let inside = &ls[catch_start..catch_start + catch_end_relative];
    assert!(inside.iter().any(|v| v.contains("\"unused catch err<br/>")));
}

// ---- let writes form a state chain ----------------------------------------

fn let_chain_code() -> String {
    [
        "function f() {",
        "  let v = 0;",
        "  v = 1;",
        "  v = 2;",
        "  return v;",
        "}",
    ]
    .join("\n")
}

#[test]
fn let_writes_chain_passes_through_one_wr_ref_node_per_assignment_in_source_order() {
    let out = emit(&let_chain_code());
    assert!(matches(&out, r"n_scope_1_v_\d+ -->\|set\| wr_ref_1"));
    assert!(matches(&out, r"wr_ref_1 -->\|set\| wr_ref_2"));
    assert!(matches(&out, r"wr_ref_2 -->\|read\| ret_use_\w+"));
}

#[test]
fn let_writes_chain_there_is_no_v_to_v_self_loop() {
    let out = emit(&let_chain_code());
    assert!(!matches(&out, r"n_scope_1_v_\d+ -->\|.*\| n_scope_1_v_\d+"));
}

#[test]
fn let_writes_chain_declaration_uses_a_rectangle_and_write_ops_use_stadium() {
    let out = emit(&let_chain_code());
    assert!(matches(&out, r#"n_scope_1_v_\d+\["let v<br/>L2"\]"#));
    assert!(matches(&out, r#"wr_ref_1\(\["let v<br/>L3"\]\)"#));
    assert!(matches(&out, r#"wr_ref_2\(\["let v<br/>L4"\]\)"#));
}

// ---- case labels ----------------------------------------------------------

#[test]
fn case_labels_numeric_and_identifier_case_tests_are_rendered_verbatim() {
    let out = emit(
        "const X = 1; let l = 0; const k = 1; switch (k) { case 0: l = 1; break; case X: l = 2; break; }\n",
    );
    assert!(matches(&out, r"case 0 L\d+"));
    assert!(matches(&out, r"case X L\d+"));
}

#[test]
fn case_labels_the_default_clause_label_is_just_default() {
    let out = emit("let l = \"\"; switch (1) { case 1: l = \"a\"; break; default: l = \"b\"; }\n");
    assert!(matches(&out, r"default L\d+"));
    assert!(!out.contains("case default"));
}

// ---- destructuring fan-out ------------------------------------------------

fn destructuring_code() -> String {
    [
        "const source = { a: 1, b: 2, nested: { d: 4 } };",
        "const list = [10, 20, 30];",
        "const { a, b: renamed } = source;",
        "const { nested: { d } } = source;",
        "const [first, , third] = list;",
        "const sum = a + renamed + d + first + third;",
    ]
    .join("\n")
}

#[test]
fn destructuring_object_source_fans_out_to_every_named_renamed_deep_binding_individually() {
    let out = emit(&destructuring_code());
    assert!(matches(
        &out,
        r"n_scope_0_source_\d+ -->\|read\| n_scope_0_a_\d+"
    ));
    assert!(matches(
        &out,
        r"n_scope_0_source_\d+ -->\|read\| n_scope_0_renamed_\d+"
    ));
    assert!(matches(
        &out,
        r"n_scope_0_source_\d+ -->\|read\| n_scope_0_d_\d+"
    ));
}

#[test]
fn destructuring_array_source_fans_out_to_its_positional_bindings_never_to_object_bindings() {
    let out = emit(&destructuring_code());
    assert!(matches(
        &out,
        r"n_scope_0_list_\d+ -->\|read\| n_scope_0_first_"
    ));
    assert!(matches(
        &out,
        r"n_scope_0_list_\d+ -->\|read\| n_scope_0_third_"
    ));
    assert!(!matches(
        &out,
        r"n_scope_0_list_\d+ -->\|read\| n_scope_0_renamed_"
    ));
    assert!(!matches(
        &out,
        r"n_scope_0_list_\d+ -->\|read\| n_scope_0_d_\d+"
    ));
}

// ---- import label prefix rule ---------------------------------------------

fn import_label_code() -> String {
    [
        "import def from 'some-default';",
        "import { named, other as renamed } from 'some-named';",
        "import * as ns from 'some-namespace';",
        "void def; void named; void renamed; void ns;",
    ]
    .join("\n")
}

#[test]
fn import_label_default_imports_get_an_import_prefix_on_the_local_node() {
    let out = emit(&import_label_code());
    assert!(matches(&out, r#"n_scope_0_def_\d+\["import def<br/>L1"\]"#));
}

#[test]
fn import_label_named_imports_whose_local_name_matches_keep_the_import_prefix() {
    let out = emit(&import_label_code());
    assert!(matches(
        &out,
        r#"n_scope_0_named_\d+\["import named<br/>L2"\]"#
    ));
}

#[test]
fn import_label_renamed_named_imports_drop_the_prefix_on_the_local_node() {
    let out = emit(&import_label_code());
    assert!(matches(
        &out,
        r#"n_scope_0_renamed_\d+\["renamed<br/>L2"\]"#
    ));
    assert!(!matches(
        &out,
        r#"n_scope_0_renamed_\d+\["import renamed<br/>"#
    ));
    assert!(matches(
        &out,
        r#"import_some_named__other\["import other<br/>L2"\]"#
    ));
}

#[test]
fn import_label_namespace_imports_get_an_import_prefix_on_the_local_node() {
    let out = emit(&import_label_code());
    assert!(matches(&out, r#"n_scope_0_ns_\d+\["import ns<br/>L3"\]"#));
}

// ---- per-depth subgraph coloring ------------------------------------------

#[test]
fn per_depth_coloring_top_level_function_wraps_in_nest_l1_and_body_in_nest_l2() {
    let out = emit("function f() { return 1; }\n");
    assert!(matches(&out, r"(?m)^\s*classDef nestL1 "));
    assert!(matches(&out, r"(?m)^\s*class wrap_s_scope_\d+ nestL1;"));
    assert!(matches(&out, r"(?m)^\s*classDef nestL2 "));
    assert!(matches(&out, r"(?m)^\s*class s_scope_\d+ nestL2;"));
}

#[test]
fn per_depth_coloring_three_levels_of_nested_if_subgraphs_emit_nest_l1_l2_l3_rows() {
    let code = [
        "function f() {",
        "  let v = 0;",
        "  const a = 1;",
        "  const b = 2;",
        "  const c = 3;",
        "  if (a) {",
        "    if (b) {",
        "      if (c) {",
        "        v = 1;",
        "      }",
        "    }",
        "  }",
        "  return v;",
        "}",
    ]
    .join("\n");
    let out = emit(&code);
    assert!(matches(&out, r"(?m)^\s*classDef nestL1 "));
    assert!(matches(&out, r"(?m)^\s*classDef nestL2 "));
    assert!(matches(&out, r"(?m)^\s*classDef nestL3 "));
}

#[test]
fn per_depth_coloring_light_theme_produces_its_own_nest_palette_colors() {
    let out = emit_with("function f() { return 1; }\n", &LIGHT_THEME, Language::Ts);
    let palette0 = &LIGHT_THEME.nest_palette[0];
    let expected = format!(
        "  classDef nestL1 fill:{},stroke:{};",
        palette0.fill, palette0.stroke
    );
    assert!(out.contains(&expected));
}

// ---- function parameters are not duplicated -------------------------------

#[test]
fn function_parameters_each_parameter_renders_exactly_one_read_edge_into_its_return_use_node() {
    let out = emit("function add(a, b) { return a + b; }\n");
    assert_eq!(
        count_matches(&out, r"n_scope_1_a_\d+ -->\|read\| ret_use_\w+"),
        1
    );
    assert_eq!(
        count_matches(&out, r"n_scope_1_b_\d+ -->\|read\| ret_use_\w+"),
        1
    );
}
