//! Sibling tests for [`emit_callback_result_edges`].

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::{
    SerializedCallbackArgument, SerializedHeadExpression, SerializedReference, SerializedScope,
};

use super::emit_callback_result_edges;
use crate::builder::builder_fixtures::{
    base_builder_context, base_serialized_reference, base_serialized_scope, empty_serialized_ir,
    scope_id, span_at, variable_id,
};
use crate::builder::state::BuildState;

/// A function (callback) scope `id` nested under `upper`, whose body
/// span starts at `block_offset`, annotated as the `arg_index`-th
/// argument of the call described by `callee`.
fn callback_scope(
    id: &str,
    upper: &str,
    block_offset: u32,
    callee: SerializedHeadExpression,
) -> SerializedScope {
    let mut s = base_serialized_scope(id);
    s.r#type = ScopeType::Function;
    s.upper = Some(scope_id(upper));
    s.block.span = span_at(1, block_offset, block_offset);
    s.callback_argument = Some(SerializedCallbackArgument {
        callee,
        arg_index: 0,
    });
    s
}

/// `recv.property` head subtree.
fn member(recv: &str, property: &str) -> SerializedHeadExpression {
    SerializedHeadExpression::member(
        SerializedHeadExpression::identifier(recv.to_string()),
        property.to_string(),
    )
}

/// A reference to `name` at `offset`, evaluated in scope `from`,
/// owning `owners`, with receiver / call roles set per the flags.
fn anchor_reference(
    name: &str,
    from: &str,
    offset: u32,
    receiver: bool,
    call: bool,
    owners: &[&str],
) -> SerializedReference {
    let mut r = base_serialized_reference();
    r.identifier = unsnarl_ir::serialized::SerializedReferenceIdentifier::new(
        name.to_string(),
        span_at(1, offset, offset),
    );
    r.from = scope_id(from);
    r.resolved = Some(variable_id(name));
    r.owners = owners.iter().map(|o| variable_id(o)).collect();
    r.flags.read = true;
    r.flags.receiver = receiver;
    r.flags.call = call;
    r
}

fn run(ir: &unsnarl_ir::serialized::SerializedIR) -> BuildState {
    let ctx = base_builder_context(ir);
    let mut state = BuildState::new();
    emit_callback_result_edges(&mut state, &ctx);
    state
}

#[test]
fn connects_method_callback_to_result_variable_with_method_label() {
    // const xs = arr.map((v) => v.id)
    let mut ir = empty_serialized_ir();
    ir.scopes = vec![callback_scope("cb", "s1", 30, member("arr", "map"))];
    ir.references = vec![anchor_reference("arr", "s1", 20, true, false, &["xs"])];

    let state = run(&ir);

    assert_eq!(state.edges.len(), 1);
    let e = &state.edges[0];
    assert_eq!(e.from, "s_cb");
    assert_eq!(e.to, "n_xs");
    assert_eq!(e.label, "map");
}

#[test]
fn connects_bare_call_callback_to_result_variable_with_function_label() {
    // const out = run((v) => v.id)
    let mut ir = empty_serialized_ir();
    ir.scopes = vec![callback_scope(
        "cb",
        "s1",
        30,
        SerializedHeadExpression::identifier("run".to_string()),
    )];
    ir.references = vec![anchor_reference("run", "s1", 20, false, true, &["out"])];

    let state = run(&ir);

    assert_eq!(state.edges.len(), 1);
    let e = &state.edges[0];
    assert_eq!(e.from, "s_cb");
    assert_eq!(e.to, "n_out");
    assert_eq!(e.label, "run");
}

#[test]
fn emits_no_edge_when_anchor_has_no_owner() {
    // await xs.reduce((p, c) => ..., init) -- bare statement, the
    // receiver reference owns nothing.
    let mut ir = empty_serialized_ir();
    ir.scopes = vec![callback_scope("cb", "s1", 30, member("xs", "reduce"))];
    ir.references = vec![anchor_reference("xs", "s1", 20, true, false, &[])];

    let state = run(&ir);

    assert!(state.edges.is_empty());
}

#[test]
fn emits_no_edge_for_chained_callee_object() {
    // const ys = xs.map(f).filter((v) => v) -- the filter callback's
    // callee object is itself a call, so there is no single owning
    // variable reference to borrow owners from.
    let mut ir = empty_serialized_ir();
    let callee = SerializedHeadExpression::member(
        SerializedHeadExpression::Call {
            callee: Box::new(member("xs", "map")),
            start_span: span_at(1, 10, 10),
            end_span: span_at(1, 18, 18),
        },
        "filter".to_string(),
    );
    ir.scopes = vec![callback_scope("cb", "s1", 30, callee)];
    ir.references = vec![anchor_reference("xs", "s1", 20, true, false, &["ys"])];

    let state = run(&ir);

    assert!(state.edges.is_empty());
}

#[test]
fn picks_nearest_preceding_anchor_when_name_repeats() {
    // Two `arr.map(...)` calls assigned to different variables; the
    // callback must bind to the receiver immediately preceding it.
    let mut ir = empty_serialized_ir();
    ir.scopes = vec![callback_scope("cb", "s1", 30, member("arr", "map"))];
    ir.references = vec![
        anchor_reference("arr", "s1", 10, true, false, &["first"]),
        anchor_reference("arr", "s1", 20, true, false, &["second"]),
    ];

    let state = run(&ir);

    assert_eq!(state.edges.len(), 1);
    assert_eq!(state.edges[0].to, "n_second");
}

#[test]
fn skips_collapsed_callback_scope() {
    let mut ir = empty_serialized_ir();
    ir.scopes = vec![callback_scope("cb", "s1", 30, member("arr", "map"))];
    ir.references = vec![anchor_reference("arr", "s1", 20, true, false, &["xs"])];

    let ctx = base_builder_context(&ir);
    let mut state = BuildState::new();
    state
        .collapsed_root_by_scope
        .insert("cb".to_string(), "root".to_string());
    emit_callback_result_edges(&mut state, &ctx);

    assert!(state.edges.is_empty());
}
