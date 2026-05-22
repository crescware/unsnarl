//! The TS suite drives `FlatSerializer` through the full
//! parse -> analyse -> serialise pipeline. The Rust port reuses the
//! same end-to-end pipeline (via `unsnarl::pipeline::emit_ir_text`)
//! and inspects the rendered IR JSON because that is what the public
//! pipeline surfaces; `FlatSerializer` itself is a thin layer that
//! the JSON output reflects byte-for-byte.

use serde_json::Value;
use unsnarl::pipeline::emit_ir_text;
use unsnarl_ir::Language;

fn pipe(code: &str) -> Value {
    let text = emit_ir_text(code, "input.ts", Language::Ts, true, &[])
        .expect("pipeline must succeed on the test inputs");
    serde_json::from_str(&text).expect("IR emitter must produce valid JSON")
}

#[test]
fn emits_version_1_ir_with_the_source_metadata() {
    let ir = pipe("const a = 1;\n");
    assert_eq!(ir["version"], 1);
    assert_eq!(ir["source"]["path"], "input.ts");
    assert_eq!(ir["source"]["language"], "ts");
}

#[test]
fn assigns_deterministic_scope_and_variable_ids() {
    let code = "const a = 1;\nconst b = a;\n";
    let ir1 = pipe(code);
    let ir2 = pipe(code);
    assert_eq!(ir1, ir2);
    assert_eq!(ir1["scopes"][0]["id"], "scope#0");
    let variable_ids: Vec<String> = ir1["variables"]
        .as_array()
        .expect("variables array")
        .iter()
        .map(|v| v["id"].as_str().expect("id string").to_string())
        .collect();
    assert_eq!(variable_ids, vec!["scope#0:a@6", "scope#0:b@19"]);
}

#[test]
fn orders_references_by_source_offset_and_assigns_ref_n_ids() {
    let code = "\n      const a = 1;\n      const b = a;\n      a;\n    ";
    let ir = pipe(code);
    let refs = ir["references"].as_array().expect("references array");

    let ref_ids: Vec<String> = refs
        .iter()
        .map(|r| r["id"].as_str().expect("id string").to_string())
        .collect();
    assert_eq!(ref_ids, vec!["ref#0", "ref#1", "ref#2", "ref#3"]);

    let names: Vec<String> = refs
        .iter()
        .map(|r| {
            r["identifier"]["name"]
                .as_str()
                .expect("identifier.name string")
                .to_string()
        })
        .collect();
    assert_eq!(names, vec!["a", "b", "a", "a"]);

    let offsets: Vec<u64> = refs
        .iter()
        .map(|r| {
            r["identifier"]["span"]["offset"]
                .as_u64()
                .expect("offset number")
        })
        .collect();
    assert!(offsets[0] < offsets[1]);
    assert!(offsets[1] < offsets[2]);
    assert!(offsets[2] < offsets[3]);
}

#[test]
fn breaks_circular_references_by_linking_ids_only() {
    let ir = pipe("function f() { return f; }\n");
    let f_var = ir["variables"]
        .as_array()
        .expect("variables array")
        .iter()
        .find(|v| v["name"].as_str() == Some("f"))
        .expect("variable `f` must exist");
    // Variable.scope is a string (ScopeId, e.g. "scope#0").
    assert!(f_var["scope"].is_string());
    // Reference.resolved is a string ID pointing at the variable.
    let f_var_id = f_var["id"].as_str().expect("variable id");
    let resolved_ref = ir["references"]
        .as_array()
        .expect("references array")
        .iter()
        .find(|r| r["identifier"]["name"].as_str() == Some("f"))
        .expect("at least one reference to `f`");
    assert_eq!(resolved_ref["resolved"], f_var_id);
    // Scope.upper is also an id reference.
    let fn_scope = ir["scopes"]
        .as_array()
        .expect("scopes array")
        .iter()
        .find(|s| s["type"].as_str() == Some("function"))
        .expect("a function scope must exist for `function f`");
    assert_eq!(fn_scope["upper"], "scope#0");
}

#[test]
fn populates_flags_correctly_for_read_write_call() {
    let code =
        "\n      let x = 0;\n      function add() { return x; }\n      x = 1;\n      add();\n    ";
    let ir = pipe(code);
    let refs = ir["references"].as_array().expect("references array");

    let x_reads: Vec<&Value> = refs
        .iter()
        .filter(|r| {
            r["identifier"]["name"].as_str() == Some("x") && r["flags"]["read"] == Value::Bool(true)
        })
        .collect();
    let x_writes: Vec<&Value> = refs
        .iter()
        .filter(|r| {
            r["identifier"]["name"].as_str() == Some("x")
                && r["flags"]["write"] == Value::Bool(true)
        })
        .collect();
    assert!(!x_reads.is_empty(), "at least one read for x");
    // Two writes: the init Write at `let x = 0` and the explicit `x = 1`.
    assert_eq!(x_writes.len(), 2);

    let add_ref = refs
        .iter()
        .find(|r| r["identifier"]["name"].as_str() == Some("add"))
        .expect("reference to `add`");
    assert_eq!(add_ref["flags"]["call"], Value::Bool(true));
    assert_eq!(add_ref["flags"]["read"], Value::Bool(true));
}

#[test]
fn collects_unused_variables_but_excludes_implicit_global_variable() {
    let code = "\n      const used = 1;\n      const unused = 2;\n      console.log(used);\n    ";
    let ir = pipe(code);
    let unused_ids: Vec<String> = ir["unusedVariableIds"]
        .as_array()
        .expect("unusedVariableIds array")
        .iter()
        .map(|v| v.as_str().expect("id string").to_string())
        .collect();
    let unused_names: Vec<String> = unused_ids
        .iter()
        .map(|id| {
            ir["variables"]
                .as_array()
                .expect("variables array")
                .iter()
                .find(|v| v["id"].as_str() == Some(id))
                .map(|v| v["name"].as_str().expect("name string").to_string())
                .expect("matching variable")
        })
        .collect();
    assert_eq!(unused_names, vec!["unused"]);
    assert!(!unused_names.iter().any(|n| n == "console"));
}

#[test]
fn includes_a_let_that_is_re_assigned_but_never_read_in_unused_variable_ids() {
    let code = "let writeOnly = 1;\nwriteOnly = 2;\nconst read = 3;\nconsole.log(read);\n";
    let ir = pipe(code);
    let unused_ids: Vec<String> = ir["unusedVariableIds"]
        .as_array()
        .expect("unusedVariableIds array")
        .iter()
        .map(|v| v.as_str().expect("id string").to_string())
        .collect();
    let unused_names: Vec<String> = unused_ids
        .iter()
        .map(|id| {
            ir["variables"]
                .as_array()
                .expect("variables array")
                .iter()
                .find(|v| v["id"].as_str() == Some(id))
                .map(|v| v["name"].as_str().expect("name string").to_string())
                .expect("matching variable")
        })
        .collect();
    assert_eq!(unused_names, vec!["writeOnly"]);
}

#[test]
fn preserves_diagnostics_including_var_detected_entries() {
    let code = "var legacy = 1;\nconst x = 2;\n";
    let ir = pipe(code);
    let diags = ir["diagnostics"].as_array().expect("diagnostics array");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0]["kind"], "var-detected");
}

#[test]
fn computes_line_column_for_spans() {
    let code = "const a = 1;\nconst b = a;\n";
    let ir = pipe(code);
    let variables = ir["variables"].as_array().expect("variables array");
    let a = variables
        .iter()
        .find(|v| v["name"].as_str() == Some("a"))
        .expect("variable a");
    let b = variables
        .iter()
        .find(|v| v["name"].as_str() == Some("b"))
        .expect("variable b");
    assert_eq!(a["identifiers"][0]["line"], 1);
    assert_eq!(b["identifiers"][0]["line"], 2);
}
