//! Integration tests for [`unsnarl_visual_graph::builder::build_visual_graph`].
//!
//! Cases mirror `ts/src/visual-graph/builder/build-visual-graph.test.ts`.
//! Lives outside `crates/unsnarl-visual-graph/src/builder/` (which
//! would be the natural sibling location) because each test feeds
//! source through `unsnarl-boundary-eslint-scope::parser::OxcParser`,
//! `unsnarl-analyzer::run_analysis`, and
//! `unsnarl-emitter-ir::FlatSerializer` before invoking
//! `build_visual_graph` — those crates sit above unsnarl-visual-graph
//! in the dep graph, so adding them as dev-deps would cycle. The
//! `unsnarl` crate already depends on all of them, so the integration
//! test lands here.
//!
//! The `build` helper inlines the pipeline that lives privately in
//! `crates/unsnarl/src/pipeline.rs` (see `serialize_ir` there).

use oxc_allocator::Allocator;
use unsnarl_analyzer::run_analysis;
use unsnarl_boundary_eslint_scope::parser::{
    default_source_type_for, OxcParser, ParseOptions, SourceType,
};
use unsnarl_emitter::{IRSerializer, SerializeContext, SerializeSourceMeta};
use unsnarl_emitter_ir::FlatSerializer;
use unsnarl_ir::serialized::SerializedIR;
use unsnarl_ir::Language;
use unsnarl_visual_graph::builder::build_visual_graph::build_visual_graph;
use unsnarl_visual_graph::builder::context::BuildVisualGraphOptions;
use unsnarl_visual_graph::node_kind::NodeKind;
use unsnarl_visual_graph::subgraph_kind::SubgraphKind;
use unsnarl_visual_graph::visual_edge::VisualEdge;
use unsnarl_visual_graph::visual_element::VisualElement;
use unsnarl_visual_graph::visual_graph::VisualGraph;
use unsnarl_visual_graph::visual_node::VisualNode;
use unsnarl_visual_graph::visual_subgraph::VisualSubgraph;

fn build(code: &str, language: Language) -> VisualGraph {
    let source_path = format!("input.{}", language_ext(language));
    let source_type = default_source_type_for_path(&source_path, language);
    let allocator = Allocator::default();
    let parser = OxcParser;
    let parsed = parser
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language,
                source_path: source_path.clone(),
                source_type,
            },
        )
        .expect("parse ok");
    let analyzed = run_analysis(&parsed.program, parsed.source_type, language, parsed.raw);
    let serializer = FlatSerializer;
    let ctx = SerializeContext {
        arena: &analyzed.arena,
        root_scope: analyzed.root_scope,
        annotations: &analyzed.annotations,
        source: SerializeSourceMeta {
            path: source_path,
            language,
        },
        diagnostics: &analyzed.diagnostics,
        raw: analyzed.raw,
    };
    let ir: SerializedIR = serializer.serialize(&ctx);
    build_visual_graph(&ir, &BuildVisualGraphOptions::default())
}

fn build_ts(code: &str) -> VisualGraph {
    build(code, Language::Ts)
}

fn language_ext(language: Language) -> &'static str {
    match language {
        Language::Ts => "ts",
        Language::Tsx => "tsx",
        Language::Js => "js",
        Language::Jsx => "jsx",
    }
}

fn default_source_type_for_path(path: &str, language: Language) -> SourceType {
    if path.ends_with(".mjs") {
        return SourceType::Module;
    }
    if path.ends_with(".cjs") {
        return SourceType::Script;
    }
    default_source_type_for(language)
}

fn flatten_nodes(elements: &[VisualElement]) -> Vec<&VisualNode> {
    let mut out = Vec::new();
    for e in elements {
        match e {
            VisualElement::Node(n) => out.push(n),
            VisualElement::Subgraph(s) => out.extend(flatten_nodes(s.elements())),
        }
    }
    out
}

fn flatten_subgraphs(elements: &[VisualElement]) -> Vec<&VisualSubgraph> {
    let mut out = Vec::new();
    for e in elements {
        if let VisualElement::Subgraph(s) = e {
            out.push(s);
            out.extend(flatten_subgraphs(s.elements()));
        }
    }
    out
}

fn find_subgraphs(g: &VisualGraph, kind: SubgraphKind) -> Vec<&VisualSubgraph> {
    flatten_subgraphs(&g.elements)
        .into_iter()
        .filter(|s| s.kind() == kind)
        .collect()
}

fn find_nodes(g: &VisualGraph, kind: NodeKind) -> Vec<&VisualNode> {
    flatten_nodes(&g.elements)
        .into_iter()
        .filter(|n| n.kind() == kind)
        .collect()
}

fn node_by_name<'a>(g: &'a VisualGraph, name: &str) -> Option<&'a VisualNode> {
    flatten_nodes(&g.elements)
        .into_iter()
        .find(|n| n.name() == name)
}

fn edges_from<'a>(g: &'a VisualGraph, from: &str) -> Vec<&'a VisualEdge> {
    g.edges.iter().filter(|e| e.from == from).collect()
}

fn edges_to<'a>(g: &'a VisualGraph, to: &str) -> Vec<&'a VisualEdge> {
    g.edges.iter().filter(|e| e.to == to).collect()
}

fn child_subgraphs_of(sg: &VisualSubgraph) -> Vec<&VisualSubgraph> {
    sg.elements()
        .iter()
        .filter_map(|e| match e {
            VisualElement::Subgraph(s) => Some(s),
            _ => None,
        })
        .collect()
}

// ---- top-level structure ----

#[test]
fn top_level_metadata_mirrors_ir_source_path_language_direction_rl() {
    let g = build_ts("const a = 1;\n");
    assert_eq!(g.source.path, "input.ts");
    assert!(matches!(g.source.language, Language::Ts));
    assert!(matches!(
        g.direction,
        unsnarl_visual_graph::direction::Direction::RL
    ));
}

#[test]
fn empty_source_produces_empty_graph() {
    let g = build_ts("");
    assert!(g.elements.is_empty());
    assert!(g.edges.is_empty());
}

#[test]
fn single_const_declaration_emits_one_const_binding_no_edges() {
    let g = build_ts("const a = 1;\n");
    let nodes = find_nodes(&g, NodeKind::ConstBinding);
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].name(), "a");
    assert!(g.edges.is_empty());
}

// ---- variable nodes ----

#[test]
fn classifies_functions_classes_parameters_catch_params_imports_by_kind() {
    let g = build_ts(
        "import imp from 'x';\nfunction foo(p) { try { p; } catch (e) { e; } }\nclass Bar {}\n",
    );
    let names_default: Vec<_> = find_nodes(&g, NodeKind::DefaultImportBinding)
        .iter()
        .map(|n| n.name().to_string())
        .collect();
    assert!(names_default.contains(&"imp".to_string()));
    let names_fn: Vec<_> = find_nodes(&g, NodeKind::FunctionDeclaration)
        .iter()
        .map(|n| n.name().to_string())
        .collect();
    assert!(names_fn.contains(&"foo".to_string()));
    let names_param: Vec<_> = find_nodes(&g, NodeKind::FormalParameter)
        .iter()
        .map(|n| n.name().to_string())
        .collect();
    assert!(names_param.contains(&"p".to_string()));
    let names_catch: Vec<_> = find_nodes(&g, NodeKind::CatchParameter)
        .iter()
        .map(|n| n.name().to_string())
        .collect();
    assert!(names_catch.contains(&"e".to_string()));
    let names_class: Vec<_> = find_nodes(&g, NodeKind::ClassDeclaration)
        .iter()
        .map(|n| n.name().to_string())
        .collect();
    assert!(names_class.contains(&"Bar".to_string()));
}

#[test]
fn unused_declarations_carry_unused_flag_but_used_do_not() {
    let g = build_ts("const a = 1;\nconst b = a;\n");
    assert_eq!(node_by_name(&g, "a").map(|n| n.unused()), Some(false));
    assert_eq!(node_by_name(&g, "b").map(|n| n.unused()), Some(true));
}

#[test]
fn let_is_emitted_as_let_binding_node() {
    let g = build_ts("let a = 1;\n");
    let a = node_by_name(&g, "a").unwrap();
    assert!(matches!(a.kind(), NodeKind::LetBinding));
}

#[test]
fn const_is_emitted_as_const_binding_node() {
    let g = build_ts("const b = 2;\n");
    let b = node_by_name(&g, "b").unwrap();
    assert!(matches!(b.kind(), NodeKind::ConstBinding));
}

#[test]
fn const_initialised_by_function_expression_is_init_is_function() {
    let g = build_ts("const fn = function () {};\n");
    let n = node_by_name(&g, "fn").unwrap();
    assert!(n.init_is_function());
}

#[test]
fn implicit_global_variable_kept_as_node_for_direct_read() {
    let g = build_ts("function f() { return globalThing; }\n");
    let n = node_by_name(&g, "globalThing").unwrap();
    assert!(matches!(n.kind(), NodeKind::SyntheticImplicitGlobal));
}

#[test]
fn implicit_global_variable_kept_for_receiver_only() {
    let g = build_ts("const x = Object.keys({});\n");
    let n = node_by_name(&g, "Object").unwrap();
    assert!(matches!(n.kind(), NodeKind::SyntheticImplicitGlobal));
}

#[test]
fn named_imports_renamed_at_import_site_keep_local_name() {
    let g = build_ts("import { other as renamed } from 'm';\nvoid renamed;\n");
    let node = node_by_name(&g, "renamed").expect("renamed");
    assert!(matches!(node.kind(), NodeKind::NamedImportBinding));
    assert_eq!(node.imported_name(), Some("other"));
}

// ---- function subgraphs ----

#[test]
fn function_declaration_becomes_function_subgraph_with_owner_node_id() {
    let g = build_ts("function add(a, b) { return a + b; }\n");
    let fns = find_subgraphs(&g, SubgraphKind::Function);
    let fn_sg = fns[0];
    let owner_id = fn_sg.owner_node_id().expect("ownerNodeId");
    let owner_node = flatten_nodes(&g.elements)
        .into_iter()
        .find(|n| n.id() == owner_id)
        .expect("owner node");
    assert_eq!(owner_node.name(), "add");
    assert!(matches!(owner_node.kind(), NodeKind::FunctionDeclaration));
}

#[test]
fn function_subgraph_mirrors_owners_name_as_owner_name() {
    let g = build_ts("function add(a, b) { return a + b; }\n");
    let fn_sg = find_subgraphs(&g, SubgraphKind::Function)[0];
    assert_eq!(fn_sg.owner_name(), Some("add"));
}

#[test]
fn arrow_function_const_function_expression_const_function_decl_all_subgraph_alike() {
    for code in [
        "const fn = (p) => p;",
        "const fn = function (p) { return p; };",
        "function fn(p) { return p; }",
    ] {
        let g = build_ts(&format!("{code}\n"));
        assert_eq!(find_subgraphs(&g, SubgraphKind::Function).len(), 1);
    }
}

#[test]
fn function_subgraph_line_range_covers_whole_block() {
    let g = build_ts("function f() {\n  return 1;\n}\n");
    let fn_sg = find_subgraphs(&g, SubgraphKind::Function)[0];
    assert_eq!(fn_sg.line(), 1);
    assert_eq!(fn_sg.end_line(), Some(3));
}

#[test]
fn single_line_function_reports_end_line_equal_to_line() {
    let g = build_ts("function f() { return 1; }\n");
    let fn_sg = find_subgraphs(&g, SubgraphKind::Function)[0];
    assert_eq!(fn_sg.line(), 1);
    assert_eq!(fn_sg.end_line(), Some(1));
}

// ---- class subgraphs ----

#[test]
fn class_declaration_becomes_class_subgraph_with_class_name() {
    let g = build_ts("class Foo {}\n");
    let subs = find_subgraphs(&g, SubgraphKind::Class);
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0].class_name(), Some("Foo"));
}

#[test]
fn named_class_expression_inner_name_is_class_name() {
    let g = build_ts("const X = class Inner {};\n");
    let subs = find_subgraphs(&g, SubgraphKind::Class);
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0].class_name(), Some("Inner"));
}

#[test]
fn anonymous_class_expression_has_null_class_name() {
    let g = build_ts("const X = class {};\n");
    let subs = find_subgraphs(&g, SubgraphKind::Class);
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0].class_name(), None);
}

// ---- control subgraphs ----

#[test]
fn try_catch_finally_produces_three_sibling_subgraphs_with_ascending_lines() {
    let g = build_ts(
        &[
            "let v = 0;",
            "try {",
            "  v = 1;",
            "} catch (err) {",
            "  v = 2;",
            "} finally {",
            "  v = 3;",
            "}",
        ]
        .join("\n"),
    );
    let try_s = find_subgraphs(&g, SubgraphKind::Try)[0];
    let catch_s = find_subgraphs(&g, SubgraphKind::Catch)[0];
    let finally_s = find_subgraphs(&g, SubgraphKind::Finally)[0];
    assert_eq!((try_s.line(), try_s.end_line()), (2, Some(4)));
    assert_eq!((catch_s.line(), catch_s.end_line()), (4, Some(6)));
    assert_eq!((finally_s.line(), finally_s.end_line()), (6, Some(8)));
}

#[test]
fn if_without_else_has_no_if_else_container_predicate_flows_to_anchor() {
    let g = build_ts(
        &[
            "let counter = 0;",
            "const flag = true;",
            "if (flag) {",
            "  counter = 1;",
            "}",
        ]
        .join("\n"),
    );
    assert_eq!(find_subgraphs(&g, SubgraphKind::IfElseContainer).len(), 0);
    let if_s = find_subgraphs(&g, SubgraphKind::If)[0];
    assert_eq!(if_s.line(), 3);
    let anchor = find_nodes(&g, NodeKind::SyntheticIfStatementTest)[0];
    assert_eq!(anchor.line(), 3);
    let flag = node_by_name(&g, "flag").unwrap();
    assert!(g
        .edges
        .iter()
        .any(|e| e.from == flag.id() && e.to == anchor.id() && e.label == "read"));
}

#[test]
fn if_else_pair_lives_inside_if_else_container_spanning_both_arms() {
    let g = build_ts(
        &[
            "let counter = 0;",
            "const flag = true;",
            "if (flag) {",
            "  counter = 1;",
            "} else {",
            "  counter = 2;",
            "}",
        ]
        .join("\n"),
    );
    let container = find_subgraphs(&g, SubgraphKind::IfElseContainer)[0];
    assert_eq!(container.line(), 3);
    assert_eq!(container.end_line(), Some(7));
    assert!(container.has_else());
    let child_kinds: Vec<_> = child_subgraphs_of(container)
        .iter()
        .map(|s| s.kind())
        .collect();
    assert_eq!(child_kinds.len(), 2);
    assert!(matches!(child_kinds[0], SubgraphKind::If));
    assert!(matches!(child_kinds[1], SubgraphKind::Else));
}

#[test]
fn for_loop_body_becomes_for_subgraph() {
    let g = build_ts("for (let i = 0; i < 1; i++) { i; }\n");
    assert_eq!(find_subgraphs(&g, SubgraphKind::For).len(), 1);
}

// ---- write operations and let-chain edges ----

#[test]
fn each_let_assignment_becomes_writeop_node_and_set_chain() {
    let g = build_ts("let v = 0;\nv = 1;\nv = 2;\n");
    let writeops = find_nodes(&g, NodeKind::WriteReference);
    let names: Vec<_> = writeops.iter().map(|n| n.name().to_string()).collect();
    assert_eq!(names, vec!["v".to_string(), "v".to_string()]);
    let set_edges: Vec<_> = g.edges.iter().filter(|e| e.label == "set").collect();
    assert_eq!(set_edges.len(), 2);
}

#[test]
fn writeop_nodes_carry_declaration_kind() {
    // We cannot import VariableDeclarationKind here because
    // `unsnarl-oxc-parity` is not in this crate's [dependencies] and
    // is only reachable transitively. Comparing against the JSON tag
    // ("let") is the robust shape-only check.
    let g = build_ts("let v = 0;\nv = 1;\n");
    let wr = find_nodes(&g, NodeKind::WriteReference)[0];
    let dk = wr.declaration_kind().expect("declarationKind");
    let serialized = serde_json::to_string(&dk).expect("serialize declaration_kind");
    assert_eq!(serialized, "\"let\"");
}

#[test]
fn variable_write_routes_through_wr_ref_not_synthetic_expr_stmt() {
    let g = build_ts("let v = 0;\nv = 1;\n");
    assert_eq!(
        find_nodes(&g, NodeKind::SyntheticExpressionStatement).len(),
        0
    );
    let writeops = find_nodes(&g, NodeKind::WriteReference);
    let names: Vec<_> = writeops.iter().map(|n| n.name().to_string()).collect();
    assert_eq!(names, vec!["v".to_string()]);
}

#[test]
fn member_write_routes_through_synthetic_expr_stmt_for_base_reference() {
    let g = build_ts("class C {\n  static z = 0;\n  static {\n    C.z = 1;\n  }\n}\n");
    let expr_stmts = find_nodes(&g, NodeKind::SyntheticExpressionStatement);
    assert_eq!(expr_stmts.len(), 1);
    assert_eq!(expr_stmts[0].name(), "C.z = ...");
    let writeops = find_nodes(&g, NodeKind::WriteReference);
    assert_eq!(writeops.len(), 0);
}

// ---- read origin edges ----

#[test]
fn const_initialised_from_other_variable_produces_one_read_edge_from_source() {
    let g = build_ts("const a = 1;\nconst b = a;\n");
    let a = node_by_name(&g, "a").unwrap();
    let b = node_by_name(&g, "b").unwrap();
    assert!(g
        .edges
        .iter()
        .any(|e| e.from == a.id() && e.to == b.id() && e.label == "read"));
}

#[test]
fn call_expressions_produce_read_call_edges() {
    let g = build_ts("function f() {}\nconst x = f();\n");
    let f = node_by_name(&g, "f").unwrap();
    let x = node_by_name(&g, "x").unwrap();
    assert!(g
        .edges
        .iter()
        .any(|e| e.from == f.id() && e.to == x.id() && e.label == "read,call"));
}

#[test]
fn after_if_else_both_branches_feed_post_merge_read_of_same_variable() {
    let g = build_ts(
        &[
            "let counter = 0;",
            "const flag = true;",
            "if (flag) {",
            "  counter = 1;",
            "} else {",
            "  counter = 2;",
            "}",
            "const result = counter;",
        ]
        .join("\n"),
    );
    let result = node_by_name(&g, "result").unwrap();
    let reads: Vec<_> = edges_to(&g, result.id())
        .into_iter()
        .filter(|e| e.label == "read")
        .collect();
    assert_eq!(reads.len(), 2);
    let writeops = find_nodes(&g, NodeKind::WriteReference);
    let writeop_ids: std::collections::HashSet<_> =
        writeops.iter().map(|n| n.id().to_string()).collect();
    for r in &reads {
        assert!(writeop_ids.contains(&r.from));
    }
}

#[test]
fn if_without_else_keeps_edge_from_pre_if_state() {
    let g = build_ts(
        &[
            "let counter = 0;",
            "const flag = true;",
            "if (flag) {",
            "  counter = 1;",
            "}",
            "const result = counter;",
        ]
        .join("\n"),
    );
    let result = node_by_name(&g, "result").unwrap();
    let reads: Vec<_> = edges_to(&g, result.id())
        .into_iter()
        .filter(|e| e.label == "read")
        .collect();
    assert_eq!(reads.len(), 2);
}

// ---- return subgraphs ----

#[test]
fn return_statement_yields_return_subgraph_with_one_return_use_per_ownerless_ref() {
    let g = build_ts("function f(a, b) { return a + b; }\n");
    let ret = find_subgraphs(&g, SubgraphKind::Return)[0];
    let uses: Vec<_> = ret
        .elements()
        .iter()
        .filter_map(|e| match e {
            VisualElement::Node(n) if n.kind() == NodeKind::ReturnArgumentReference => Some(n),
            _ => None,
        })
        .collect();
    assert_eq!(uses.len(), 2);
}

#[test]
fn each_return_statement_gets_its_own_subgraph_sibling_returns_not_merged() {
    let g = build_ts(
        &[
            "function pick(k) {",
            "  const a = 1;",
            "  const b = 2;",
            "  if (k) { return a; }",
            "  return b;",
            "}",
        ]
        .join("\n"),
    );
    let returns = find_subgraphs(&g, SubgraphKind::Return);
    assert_eq!(returns.len(), 2);
    let mut lines: Vec<_> = returns.iter().map(|r| r.line()).collect();
    lines.sort();
    assert_eq!(lines, vec![4, 5]);
}

#[test]
fn function_with_no_ownerless_refs_in_body_produces_no_return_subgraph() {
    let g = build_ts("function f() { return 1; }\n");
    assert_eq!(find_subgraphs(&g, SubgraphKind::Return).len(), 0);
}

#[test]
fn single_line_return_subgraph_leaves_end_line_null() {
    let g = build_ts("function f(x) { return x; }\n");
    let ret = find_subgraphs(&g, SubgraphKind::Return)[0];
    assert_eq!(ret.line(), 1);
    assert_eq!(ret.end_line(), None);
}

#[test]
fn ownerless_refs_flow_into_return_use_via_read_edges() {
    let g = build_ts("function f(a) { return a; }\n");
    let a = node_by_name(&g, "a").unwrap();
    let ret = find_subgraphs(&g, SubgraphKind::Return)[0];
    let ret_use = ret
        .elements()
        .iter()
        .find_map(|e| match e {
            VisualElement::Node(n) if n.kind() == NodeKind::ReturnArgumentReference => Some(n),
            _ => None,
        })
        .expect("ret use");
    assert!(g
        .edges
        .iter()
        .any(|e| e.from == a.id() && e.to == ret_use.id() && e.label == "read"));
}

#[test]
fn non_jsx_return_use_stays_is_jsx_element_false_end_line_null() {
    let g = build_ts("function f(a) { return a; }\n");
    let ret = find_subgraphs(&g, SubgraphKind::Return)[0];
    let ret_use = ret
        .elements()
        .iter()
        .find_map(|e| match e {
            VisualElement::Node(n) if n.kind() == NodeKind::ReturnArgumentReference => Some(n),
            _ => None,
        })
        .expect("ret use");
    assert!(!ret_use.is_jsx_element());
    assert_eq!(ret_use.end_line(), None);
}

// ---- imports ----

#[test]
fn default_imports_get_single_module_source_and_read_edge_to_local_binding() {
    let g = build_ts("import def from 'lib';\nvoid def;\n");
    let module_source = find_nodes(&g, NodeKind::SyntheticModuleSource)[0];
    assert_eq!(module_source.name(), "lib");
    let def = node_by_name(&g, "def").unwrap();
    assert!(g
        .edges
        .iter()
        .any(|e| e.from == module_source.id() && e.to == def.id()));
}

#[test]
fn renamed_named_imports_introduce_intermediate_carrying_original_name() {
    let g = build_ts("import { other as renamed } from 'lib';\nvoid renamed;\n");
    let intermediates = find_nodes(&g, NodeKind::SyntheticImportIntermediate);
    assert_eq!(intermediates.len(), 1);
    assert_eq!(intermediates[0].name(), "other");
    let module_source = find_nodes(&g, NodeKind::SyntheticModuleSource)[0];
    let renamed = node_by_name(&g, "renamed").unwrap();
    assert!(g
        .edges
        .iter()
        .any(|e| e.from == module_source.id() && e.to == intermediates[0].id()));
    assert!(g
        .edges
        .iter()
        .any(|e| e.from == intermediates[0].id() && e.to == renamed.id()));
}

#[test]
fn namespace_imports_point_directly_from_module_to_local_binding() {
    let g = build_ts("import * as ns from 'lib';\nvoid ns;\n");
    assert_eq!(
        find_nodes(&g, NodeKind::SyntheticImportIntermediate).len(),
        0
    );
    let module_source = find_nodes(&g, NodeKind::SyntheticModuleSource)[0];
    let ns = node_by_name(&g, "ns").unwrap();
    assert!(g
        .edges
        .iter()
        .any(|e| e.from == module_source.id() && e.to == ns.id()));
}

// ---- predicate references ----

#[test]
fn switch_discriminant_identifier_feeds_switch_discriminant_anchor() {
    let g = build_ts(
        &[
            "let l = 0;",
            "const k = 1;",
            "switch (k) { case 1: l = 1; break; default: l = 2; }",
        ]
        .join("\n"),
    );
    let k = node_by_name(&g, "k").unwrap();
    let anchor = find_nodes(&g, NodeKind::SyntheticSwitchStatementDiscriminant)[0];
    assert!(g
        .edges
        .iter()
        .any(|e| e.from == k.id() && e.to == anchor.id() && e.label == "read"));
}

#[test]
fn bare_if_predicate_identifier_feeds_if_test_anchor() {
    let g = build_ts(&["let v = 0;", "const flag = true;", "if (flag) { v = 1; }"].join("\n"));
    let flag = node_by_name(&g, "flag").unwrap();
    let anchor = find_nodes(&g, NodeKind::SyntheticIfStatementTest)[0];
    assert!(g
        .edges
        .iter()
        .any(|e| e.from == flag.id() && e.to == anchor.id() && e.label == "read"));
}

// ---- ownerless refs at module scope ----

#[test]
fn top_level_expression_statement_gets_own_node_with_call_head_and_line() {
    let g = build_ts("const a = 1;\nconsole.log(a);\n");
    let expr_node = find_nodes(&g, NodeKind::SyntheticExpressionStatement)[0];
    assert_eq!(expr_node.name(), "console.log()");
    assert_eq!(expr_node.line(), 2);
    let a = node_by_name(&g, "a").unwrap();
    assert!(edges_from(&g, a.id())
        .iter()
        .any(|e| e.to == expr_node.id()));
    let console_node = node_by_name(&g, "console").unwrap();
    assert!(edges_from(&g, console_node.id())
        .iter()
        .any(|e| e.to == expr_node.id()));
}

#[test]
fn non_call_top_level_expr_stmt_uses_bare_expression_as_head() {
    let g = build_ts("const a = 1;\na;\n");
    let expr_node = find_nodes(&g, NodeKind::SyntheticExpressionStatement)[0];
    assert_eq!(expr_node.name(), "a");
    assert_eq!(expr_node.line(), 2);
}

// ---- var declarations ----

#[test]
fn var_declared_variables_emit_node_but_no_edges() {
    let g = build_ts("var v = 0;\nconsole.log(v);\n");
    let var_node = node_by_name(&g, "v").expect("v");
    assert!(matches!(var_node.kind(), NodeKind::VarBinding));
    assert_eq!(edges_from(&g, var_node.id()).len(), 0);
    assert_eq!(edges_to(&g, var_node.id()).len(), 0);
    let writeops: Vec<_> = flatten_nodes(&g.elements)
        .into_iter()
        .filter(|n| n.kind() == NodeKind::WriteReference && n.name() == "v")
        .collect();
    assert_eq!(writeops.len(), 0);
}

#[test]
fn unused_var_node_is_not_flagged_unused() {
    let g = build_ts("var w = 0;\n");
    let var_node = node_by_name(&g, "w").unwrap();
    assert!(!var_node.unused());
}

// ---- edge deduplication ----

#[test]
fn same_logical_read_is_emitted_only_once_per_destination() {
    let g = build_ts("function f(a) { return a + a; }\n");
    let a = node_by_name(&g, "a").unwrap();
    let ret = find_subgraphs(&g, SubgraphKind::Return)[0];
    let uses: Vec<_> = ret
        .elements()
        .iter()
        .filter_map(|e| match e {
            VisualElement::Node(n)
                if n.kind() == NodeKind::ReturnArgumentReference && n.name() == "a" =>
            {
                Some(n)
            }
            _ => None,
        })
        .collect();
    assert!(!uses.is_empty());
    let use_ids: std::collections::HashSet<_> = uses.iter().map(|n| n.id().to_string()).collect();
    let edges: Vec<_> = edges_from(&g, a.id())
        .into_iter()
        .filter(|e| use_ids.contains(&e.to))
        .collect();
    assert_eq!(edges.len(), uses.len());
}
