//! Sibling tests for [`callback_result_target`].

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::{
    SerializedCallbackArgument, SerializedDefinition, SerializedHeadExpression,
    SerializedReference, SerializedScope, SerializedVariable, VariableDef,
};
use unsnarl_oxc_parity::{AstType, VariableDeclarationKind};

use super::callback_result_target;
use crate::builder::builder_fixtures::{
    base_builder_context, base_serialized_reference, base_serialized_scope, definition_name,
    definition_node, empty_serialized_ir, scope_id, span_at, variable_id,
};

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

fn member(recv: &str, property: &str) -> SerializedHeadExpression {
    SerializedHeadExpression::member(
        SerializedHeadExpression::identifier(recv.to_string()),
        property.to_string(),
    )
}

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

/// A `const`-bound result variable whose declarator init (the call)
/// starts at `init_offset`.
fn result_var(id: &str, init_offset: u32) -> SerializedVariable {
    let def = SerializedDefinition::Variable(VariableDef::new(
        definition_name(id, span_at(1, 1, 1)),
        definition_node(AstType::Identifier, span_at(1, 1, 1)),
        None,
        Some(definition_node(
            AstType::CallExpression,
            span_at(1, init_offset, init_offset),
        )),
        VariableDeclarationKind::Const,
    ));
    SerializedVariable::new(
        variable_id(id),
        id.to_string(),
        scope_id("s1"),
        vec![span_at(1, 1, 1)],
        Vec::new(),
        vec![def],
    )
}

#[test]
fn resolves_method_receiver_result_variable() {
    // const xs = arr.map((v) => v.id)
    let mut ir = empty_serialized_ir();
    ir.variables = vec![result_var("xs", 15)];
    ir.scopes = vec![callback_scope("cb", "s1", 30, member("arr", "map"))];
    ir.references = vec![anchor_reference("arr", "s1", 20, true, false, &["xs"])];
    let ctx = base_builder_context(&ir);

    let target = callback_result_target(&ir.scopes[0], &ctx).expect("resolves");
    assert_eq!(target.owner_var_id, "xs");
}

#[test]
fn resolves_bare_call_result_variable() {
    // const out = run((v) => v.id)
    let mut ir = empty_serialized_ir();
    ir.variables = vec![result_var("out", 15)];
    ir.scopes = vec![callback_scope(
        "cb",
        "s1",
        30,
        SerializedHeadExpression::identifier("run".to_string()),
    )];
    ir.references = vec![anchor_reference("run", "s1", 20, false, true, &["out"])];
    let ctx = base_builder_context(&ir);

    let target = callback_result_target(&ir.scopes[0], &ctx).expect("resolves");
    assert_eq!(target.owner_var_id, "out");
}

#[test]
fn none_when_anchor_has_no_owner() {
    // await xs.reduce((p, c) => ..., init) -- bare statement.
    let mut ir = empty_serialized_ir();
    ir.scopes = vec![callback_scope("cb", "s1", 30, member("xs", "reduce"))];
    ir.references = vec![anchor_reference("xs", "s1", 20, true, false, &[])];
    let ctx = base_builder_context(&ir);

    assert!(callback_result_target(&ir.scopes[0], &ctx).is_none());
}

#[test]
fn none_for_chained_callee_object() {
    // const ys = xs.map(f).filter((v) => v) -- callee object is a call.
    let mut ir = empty_serialized_ir();
    ir.variables = vec![result_var("ys", 15)];
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
    let ctx = base_builder_context(&ir);

    assert!(callback_result_target(&ir.scopes[0], &ctx).is_none());
}

#[test]
fn none_for_multi_owner_destructure() {
    // const [a, b] = pair.map((v) => v) -- two result bindings.
    let mut ir = empty_serialized_ir();
    ir.scopes = vec![callback_scope("cb", "s1", 30, member("pair", "map"))];
    ir.references = vec![anchor_reference("pair", "s1", 20, true, false, &["a", "b"])];
    let ctx = base_builder_context(&ir);

    assert!(callback_result_target(&ir.scopes[0], &ctx).is_none());
}

#[test]
fn none_for_nested_call_in_argument() {
    // const images = getSummaries(app, data.map((v) => v.id))
    // `data` owns `images` (data flows in), but the call bound to
    // `images` is `getSummaries(...)`, not `data.map(...)`. The outer
    // callee `getSummaries` starts earlier, so the inner callback is
    // not a direct result-bound call.
    let mut ir = empty_serialized_ir();
    ir.variables = vec![result_var("images", 15)];
    ir.scopes = vec![callback_scope("cb", "s1", 30, member("data", "map"))];
    ir.references = vec![
        anchor_reference("getSummaries", "s1", 16, false, true, &["images"]),
        anchor_reference("data", "s1", 25, true, false, &["images"]),
    ];
    let ctx = base_builder_context(&ir);

    assert!(callback_result_target(&ir.scopes[0], &ctx).is_none());
}

#[test]
fn picks_nearest_preceding_anchor() {
    let mut ir = empty_serialized_ir();
    ir.variables = vec![result_var("second", 15)];
    ir.scopes = vec![callback_scope("cb", "s1", 30, member("arr", "map"))];
    ir.references = vec![
        anchor_reference("arr", "s1", 10, true, false, &["first"]),
        anchor_reference("arr", "s1", 20, true, false, &["second"]),
    ];
    let ctx = base_builder_context(&ir);

    let target = callback_result_target(&ir.scopes[0], &ctx).expect("resolves");
    assert_eq!(target.owner_var_id, "second");
}
