//! Sibling tests for [`build_analysis_visitor`].
//!
//! Most analyzer behavior is covered by per-helper sibling tests
//! (`block_context_of_test.rs`, `find_completion_test.rs`, ...) that
//! poke the helpers directly with fabricated [`PathEntry`] fixtures.
//! The walker-level tests here exercise [`run_analysis`] end-to-end
//! where the helper-level tests cannot see the slot-key bookkeeping
//! the walker is responsible for pushing.

use oxc_allocator::Allocator;
use oxc_parser::{Parser, ParserReturn};
use oxc_span::SourceType;
use unsnarl_annotations::Annotations;
use unsnarl_oxc_boundary::parser::SourceType as BoundarySourceType;

use unsnarl_ir::reference::reference_flags::ReferenceFlags;
use unsnarl_ir::Language;

use crate::run_analysis::run_analysis;

#[test]
fn export_named_declaration_routes_declaration_slot_key_into_block_context() {
    // Parity regression: when the boundary's walker reached
    // `export class Foo {}`, the analyzer used to emit
    // `blockContext.key = "body"` on the inner class scope --
    // inheriting the surrounding `Program.body` slot label -- because
    // oxc's auto-generated `walk_export_named_declaration` does not
    // record any per-child key. The npm `oxc-parser` package keys the
    // declaration child as `"declaration"` (per its `visitorKeys`
    // map), and the IR must match byte-for-byte.
    let source = "export class Foo {}\n";
    let allocator = Allocator::default();
    let ParserReturn { program, .. } = Parser::new(&allocator, source, SourceType::ts()).parse();

    let analyzed = run_analysis(&program, BoundarySourceType::Module, Language::Ts, source);

    let class_scope = analyzed
        .arena
        .scopes
        .iter_enumerated()
        .find(|(_, s)| matches!(s.r#type, unsnarl_ir::scope_type::ScopeType::Class))
        .map(|(id, _)| id)
        .expect("class scope must exist for `export class Foo {}`");

    let block_context = analyzed
        .annotations
        .of_scope(class_scope)
        .block_context
        .as_ref()
        .expect("class scope must have a block_context");
    let bc = serde_json::to_value(block_context).expect("BlockContext serializes to JSON");
    assert_eq!(bc["kind"], "other");
    assert_eq!(bc["parentType"], "ExportNamedDeclaration");
    assert_eq!(
        bc["key"], "declaration",
        "the class scope's blockContext.key must be \"declaration\", \
         not the surrounding statement-list's \"body\""
    );
}

#[test]
fn assignment_target_owners_match_reference_resolved_when_var_hoisted_from_later_sibling_block() {
    // Parity regression discovered against cytoscape.min.js: when an
    // assignment target identifier shadows a `var` binding hoisted
    // from a later sibling block, the analyzer used to compute
    // `owners` via a fresh `resolve_in_scope_chain` call. By the time
    // the analysis pass runs the boundary has already hoisted every
    // `var` into the function scope's `set`, so the call returns the
    // local binding -- but at the moment the corresponding reference
    // fires inline with scope-build, only the hoistings encountered
    // SO FAR are visible, and the reference resolves to the outer
    // (implicit-global) binding instead. The analyzer must report
    // what was visible at reference-binding time, which is exactly
    // the value already stored on the reference's `.resolved` field.
    //
    // In the input below, `y = 1` at the top of `f` writes through
    // an implicit-global `y` (created in the global scope at the
    // assignment site, offset 17) because no local `y` is in scope
    // when the reference fires. The `var y` inside the `for` block
    // hoists to the function scope only after the assignment has
    // already been bound. Owners on the assignment's write reference
    // must therefore equal the reference's resolved value -- the
    // implicit global at offset 17 -- not the function-scope `y`
    // declared inside the later block.
    let source = "function f() {\n  y = 1;\n  for (var u = 0; u < 1; u++) { var y; }\n}\n";
    let allocator = Allocator::default();
    let ParserReturn { program, .. } = Parser::new(&allocator, source, SourceType::ts()).parse();

    let analyzed = run_analysis(&program, BoundarySourceType::Script, Language::Ts, source);

    let (write_ref_id, write_ref) = analyzed
        .arena
        .references
        .iter_enumerated()
        .find(|(_, r)| {
            r.identifier.name() == "y"
                && r.identifier.span.start == 17
                && (r.flags & ReferenceFlags::WRITE).0 != 0
                && (r.flags & ReferenceFlags::READ).0 == 0
        })
        .expect("write reference for `y = 1` must exist at offset 17");

    let resolved = write_ref
        .resolved
        .expect("write reference must resolve to a variable (implicit-global y)");

    let owners = &analyzed.annotations.of_reference(write_ref_id).owners;
    assert_eq!(
        owners.as_slice(),
        &[resolved],
        "owners on the write reference must equal [reference.resolved] \
         (implicit-global y@17), not the function-scope `var y` hoisted \
         from the later for-block"
    );
}

#[test]
fn block_inside_labeled_statement_reports_body_slot_key() {
    // Parity regression: when a BlockStatement is the body of a
    // LabeledStatement, the block scope's `blockContext.key` used to
    // come out as `"consequent"` (the slot label inherited from
    // whatever ambient parent was on the visitor's key stack --
    // typically an enclosing IfStatement). The ESTree slot label is
    // `"body"` and the IR must match.
    let source = "if (true) outer: { let x = 1; }\n";
    let allocator = Allocator::default();
    let ParserReturn { program, .. } = Parser::new(&allocator, source, SourceType::ts()).parse();

    let analyzed = run_analysis(&program, BoundarySourceType::Script, Language::Ts, source);

    let block_scope = analyzed
        .arena
        .scopes
        .iter_enumerated()
        .find(|(_, s)| matches!(s.r#type, unsnarl_ir::scope_type::ScopeType::Block))
        .map(|(id, _)| id)
        .expect("a block scope must exist for `{ let x = 1; }`");

    let block_context = analyzed
        .annotations
        .of_scope(block_scope)
        .block_context
        .as_ref()
        .expect("the block scope must carry a block_context");
    let bc = serde_json::to_value(block_context).expect("BlockContext serialises to JSON");
    assert_eq!(bc["parentType"], "LabeledStatement");
    assert_eq!(
        bc["key"], "body",
        "the block's blockContext.key must carry the ESTree slot \
         label `body`, not the auto-walker's `consequent` carryover"
    );
}

#[test]
fn class_inside_sequence_expression_reports_expressions_slot_key() {
    // Parity regression: when a ClassExpression participates in a
    // comma-separated SequenceExpression, the class scope's
    // `blockContext.key` used to come out as `"argument"` (carried
    // over from an enclosing argument-shaped parent in the walker's
    // key stack). The ESTree slot label is `"expressions"`.
    let source = "(class A {}, class B {});\n";
    let allocator = Allocator::default();
    let ParserReturn { program, .. } = Parser::new(&allocator, source, SourceType::ts()).parse();

    let analyzed = run_analysis(&program, BoundarySourceType::Script, Language::Ts, source);

    let class_scope = analyzed
        .arena
        .scopes
        .iter_enumerated()
        .find(|(_, s)| matches!(s.r#type, unsnarl_ir::scope_type::ScopeType::Class))
        .map(|(id, _)| id)
        .expect("a class scope must exist for the leading `class A {}`");

    let block_context = analyzed
        .annotations
        .of_scope(class_scope)
        .block_context
        .as_ref()
        .expect("the class scope must carry a block_context");
    let bc = serde_json::to_value(block_context).expect("BlockContext serialises to JSON");
    assert_eq!(bc["parentType"], "SequenceExpression");
    assert_eq!(
        bc["key"], "expressions",
        "the class's blockContext.key must carry the ESTree slot \
         label `expressions`, not the auto-walker's `argument` carryover"
    );
}

#[test]
fn export_default_declaration_routes_declaration_slot_key_into_block_context() {
    // Parity regression: when a class / function is the
    // `declaration` slot of an `export default`, the inner scope's
    // `blockContext.key` used to come out as `"body"` (carried over
    // from the surrounding `Program.body` statement-list slot)
    // because oxc's auto-generated `walk_export_default_declaration`
    // does not push a per-child key. The ESTree slot label is
    // `"declaration"` and the IR must match -- same family of bug
    // as `export class Foo {}` for ExportNamedDeclaration.
    let source = "export default class Foo {}\n";
    let allocator = Allocator::default();
    let ParserReturn { program, .. } = Parser::new(&allocator, source, SourceType::ts()).parse();

    let analyzed = run_analysis(&program, BoundarySourceType::Module, Language::Ts, source);

    let class_scope = analyzed
        .arena
        .scopes
        .iter_enumerated()
        .find(|(_, s)| matches!(s.r#type, unsnarl_ir::scope_type::ScopeType::Class))
        .map(|(id, _)| id)
        .expect("a class scope must exist for `export default class Foo {}`");

    let block_context = analyzed
        .annotations
        .of_scope(class_scope)
        .block_context
        .as_ref()
        .expect("the class scope must carry a block_context");
    let bc = serde_json::to_value(block_context).expect("BlockContext serialises to JSON");
    assert_eq!(bc["parentType"], "ExportDefaultDeclaration");
    assert_eq!(
        bc["key"], "declaration",
        "the class's blockContext.key must carry the ESTree slot \
         label `declaration`, not the auto-walker's `body` carryover"
    );
}

#[test]
fn export_named_with_from_creates_implicit_global_reference_for_local_name() {
    // Parity regression: `export { Lexer } from './Lexer.js'` is
    // expected to produce a read reference for `Lexer` resolving to
    // an implicit-global in the module scope. The Rust `oxc_parser`
    // crate represents the slot as `ModuleExportName::IdentifierName`
    // for re-exports (and anywhere the local name is a reserved word
    // like `default`), so the analyzer used to walk past it without
    // firing the reference. The boundary's `visit_identifier_name`
    // now routes these slots through `handle_identifier_reference`.
    let source = "export { Lexer } from './Lexer.js'\n";
    let allocator = Allocator::default();
    let ParserReturn { program, .. } = Parser::new(&allocator, source, SourceType::ts()).parse();

    let analyzed = run_analysis(&program, BoundarySourceType::Module, Language::Ts, source);

    let lexer_var = analyzed
        .arena
        .variables
        .iter()
        .find(|v| v.name() == "Lexer")
        .expect("an implicit-global `Lexer` variable must be created for the re-export local name");

    assert!(
        !lexer_var.references.is_empty(),
        "the implicit-global `Lexer` must carry at least one reference"
    );

    let ref_id = lexer_var.references[0];
    let reference = &analyzed.arena.references[ref_id];
    assert_eq!(reference.identifier.name(), "Lexer");
    assert_eq!(reference.identifier.span.start, 9);
}

#[test]
fn export_all_with_alias_creates_implicit_global_reference_for_exported_name() {
    // Parity regression: `export * as default from './base.js'` is
    // expected to produce an implicit-global `default` plus one read
    // reference for the alias identifier. The Rust `oxc_parser` crate
    // represents the slot as
    // `ExportAllDeclaration.exported: Option<ModuleExportName>`, and
    // for `default` (and for any plain-Identifier alias) the variant
    // is `ModuleExportName::IdentifierName`, which previously walked
    // through `visit_identifier_name` without firing the reference.
    let source = "export * as default from './base.js'\n";
    let allocator = Allocator::default();
    let ParserReturn { program, .. } = Parser::new(&allocator, source, SourceType::ts()).parse();

    let analyzed = run_analysis(&program, BoundarySourceType::Module, Language::Ts, source);

    let default_var = analyzed
        .arena
        .variables
        .iter()
        .find(|v| v.name() == "default")
        .expect("an implicit-global `default` variable must exist for the export-all alias");

    assert!(
        !default_var.references.is_empty(),
        "the implicit-global `default` must carry at least one reference"
    );
}

#[test]
fn private_field_assignment_head_renders_as_member_not_raw() {
    // Parity regression: the head expression for `this.#prop = rhs`
    // (and `obj.#prop = rhs`) must come out as
    // `{ kind: assign, left: { kind: member, property: "prop" }, ... }`,
    // i.e. the private field is flattened to a member shape carrying
    // the bare name. The Rust `oxc_parser` crate keeps
    // PrivateFieldExpression separate from StaticMemberExpression in
    // three head-building arms (`Expression`, `AssignmentTarget`,
    // `SimpleAssignmentTarget`); each of those used to bail to
    // `None`, which then collapsed the whole assignment head to
    // `kind: raw`. Flatten the private-field shape in each arm.
    // `x` is the inner reference whose container annotation we
    // inspect. The surrounding ExpressionStatement is `obj.#o = x;`,
    // so the container's head must classify the whole statement as
    // an assign with the private-field left-hand side flattened to a
    // member shape. (Using a bare identifier `obj` instead of `this`
    // because the head builder does not reduce `ThisExpression` and
    // would otherwise collapse the left operand to `Elided` -- a
    // separate parity gap that surfaces only when both sides of an
    // assignment collapse.)
    let source = "class C { #o = 1; foo(obj, x) { obj.#o = x; } }\n";
    let allocator = Allocator::default();
    let ParserReturn { program, .. } = Parser::new(&allocator, source, SourceType::ts()).parse();

    let analyzed = run_analysis(&program, BoundarySourceType::Module, Language::Ts, source);

    let x_read_ref_id = analyzed
        .arena
        .references
        .iter_enumerated()
        .find(|(_, r)| {
            r.identifier.name() == "x"
                && (r.flags & ReferenceFlags::READ).0 != 0
                && (r.flags & ReferenceFlags::WRITE).0 == 0
        })
        .map(|(id, _)| id)
        .expect("a read reference for `x` must exist on the RHS of `this.#o = x;`");

    let container = analyzed
        .annotations
        .of_reference(x_read_ref_id)
        .expression_statement_container
        .as_ref()
        .expect("`this.#o = x;` is an ExpressionStatement, so the container must be present");

    use unsnarl_ir::reference::expression_statement_head::HeadExpression;
    let HeadExpression::Assign { left, .. } = &container.head else {
        panic!(
            "expected the head of `this.#o = 2;` to be HeadExpression::Assign, got {:?}",
            std::mem::discriminant(&container.head)
        );
    };
    let HeadExpression::Member { property, .. } = &left.head else {
        panic!(
            "expected the assign's left-hand side to be HeadExpression::Member, got {:?}",
            std::mem::discriminant(&left.head)
        );
    };
    assert_eq!(
        property, "o",
        "the private field name must be flattened to the bare identifier (no leading `#`) in the head expression"
    );
}

#[test]
fn function_inside_object_property_value_slot_reports_value_block_context_key() {
    // Parity regression: when a function / class expression is the
    // `value` slot of an `ObjectProperty` (e.g. `{ key: function ()
    // {} }`) and the enclosing expression is wrapped in a call
    // argument list, the function scope's `blockContext.key` used
    // to come out as `"arguments"` -- carried over from
    // `CallExpression.arguments` -- instead of the expected
    // `"value"`. oxc's auto-generated `walk_object_property` does not
    // push a per-child slot key, so the surrounding label leaks
    // through.
    // Use a class expression rather than a function expression
    // because the analyzer's `block_context_of` only emits a block
    // context for scope types that have one (class scopes do;
    // function scopes are gated on richer body shapes than an empty
    // function expression produces).
    let source = "callMe({ key: class {} });\n";
    let allocator = Allocator::default();
    let ParserReturn { program, .. } = Parser::new(&allocator, source, SourceType::ts()).parse();

    let analyzed = run_analysis(&program, BoundarySourceType::Script, Language::Ts, source);

    let class_scope = analyzed
        .arena
        .scopes
        .iter_enumerated()
        .find(|(_, s)| matches!(s.r#type, unsnarl_ir::scope_type::ScopeType::Class))
        .map(|(id, _)| id)
        .expect("a class scope must exist for the value-slot class expression");

    let block_context = analyzed
        .annotations
        .of_scope(class_scope)
        .block_context
        .as_ref()
        .expect("the class scope must carry a block_context");
    let bc = serde_json::to_value(block_context).expect("BlockContext serialises to JSON");
    assert_eq!(bc["parentType"], "Property");
    assert_eq!(
        bc["key"], "value",
        "the class's blockContext.key must carry the ESTree slot \
         label `value`, not the auto-walker's `arguments` carryover"
    );
}

/// Run the analyzer and hand `f` the `callback_argument` annotation
/// of every `Function` scope, in source order (`None` where a scope
/// carries no annotation). Borrows from the live `Analyzed`, so the
/// callee subtree is inspected structurally without cloning or
/// rendering it to a string.
fn with_callback_args<R>(
    source: &str,
    f: impl FnOnce(Vec<Option<&unsnarl_ir::scope::CallbackArgument>>) -> R,
) -> R {
    use unsnarl_ir::scope_type::ScopeType;
    let allocator = Allocator::default();
    let ParserReturn { program, .. } = Parser::new(&allocator, source, SourceType::ts()).parse();
    let analyzed = run_analysis(&program, BoundarySourceType::Script, Language::Ts, source);
    let args: Vec<Option<&unsnarl_ir::scope::CallbackArgument>> = analyzed
        .arena
        .scopes
        .iter_enumerated()
        .filter(|(_, s)| matches!(s.r#type, ScopeType::Function))
        .map(|(id, _)| analyzed.annotations.of_scope(id).callback_argument.as_ref())
        .collect();
    f(args)
}

/// The name of a callee that is a bare `Identifier`, else `None`.
fn identifier_name(head: &unsnarl_ir::reference::HeadExpression) -> Option<&str> {
    match head {
        unsnarl_ir::reference::HeadExpression::Identifier { name } => Some(name),
        _ => None,
    }
}

/// A callee `Member`'s `(object, property)`, else `None`.
fn member_parts(
    head: &unsnarl_ir::reference::HeadExpression,
) -> Option<(&unsnarl_ir::reference::HeadExpression, &str)> {
    match head {
        unsnarl_ir::reference::HeadExpression::Member { object, property } => {
            Some((object, property))
        }
        _ => None,
    }
}

/// A `Call`'s callee subtree, else `None`.
fn call_callee(
    head: &unsnarl_ir::reference::HeadExpression,
) -> Option<&unsnarl_ir::reference::HeadExpression> {
    match head {
        unsnarl_ir::reference::HeadExpression::Call { callee, .. } => Some(callee),
        _ => None,
    }
}

#[test]
fn callback_argument_is_set_for_a_function_passed_as_a_direct_call_argument() {
    with_callback_args("run(() => {});\n", |args| {
        let cb =
            args[0].expect("a function literal in arg 0 of a call must carry a callback_argument");
        assert_eq!(cb.arg_index, 0);
        assert_eq!(identifier_name(&cb.callee), Some("run"));
    });
}

#[test]
fn callback_argument_uses_zero_based_index_for_later_argument_slots() {
    // The arrow is arg 1 of `run`, not arg 0.
    with_callback_args("run(a, () => {});\n", |args| {
        let cb = args[0].expect("arg 1 callback must be annotated");
        assert_eq!(cb.arg_index, 1);
    });
}

#[test]
fn callback_argument_is_set_for_a_callback_bound_to_a_variable() {
    // The call sits in a VariableDeclarator initializer, not an
    // ExpressionStatement. The annotation still fires for any
    // call-argument function -- the structural fact does not depend on
    // statement position. (Whether a CallProxy wrapper is rendered is
    // decided later in the visual-graph layer, not encoded here.)
    with_callback_args("const x = run(() => {});\n", |args| {
        let cb = args[0].expect("a variable-bound callback must still carry a callback_argument");
        assert_eq!(cb.arg_index, 0);
        assert_eq!(identifier_name(&cb.callee), Some("run"));
    });
}

#[test]
fn callback_argument_does_not_leak_into_the_callee_of_an_inner_call() {
    // Regression: `outer((function(){})())` puts a function expression
    // in the *callee* slot of the inner CallExpression, which itself
    // sits in arg 0 of the outer call. The arg_index_stack top remains
    // `Some(0)` while traversing into the inner callee, so without an
    // explicit `current_key == Some("arguments")` guard the IIFE
    // function scope would be misannotated as `outer`'s arg 0.
    with_callback_args("outer((function () {})());\n", |args| {
        let cb = args[0];
        assert!(
            cb.is_none(),
            "an IIFE function in callee position must not be annotated as the outer call's arg: {:?}",
            cb.map(|c| c.arg_index)
        );
    });
}

#[test]
fn callback_argument_is_set_for_a_constructor_callback() {
    // Annotation must fire for `NewExpression` parents too -- the
    // analyzer treats `new Service(cb)` the same as `service(cb)`
    // for callback-arg purposes.
    with_callback_args("new Service(() => {});\n", |args| {
        let cb = args[0].expect("a constructor callback must be annotated");
        assert_eq!(cb.arg_index, 0);
        assert_eq!(identifier_name(&cb.callee), Some("Service"));
    });
}

#[test]
fn callback_argument_captures_distinct_callees_for_calls_in_a_chain() {
    // Each callback carries its own call's `callee` head, so two
    // callbacks in the same chain are distinguished by the structure of
    // that subtree rather than by a `(start, end)` offset pair.
    // `.then(cb)`'s callee is `Promise.resolve().then`; `.catch(cb)`'s
    // callee is `Promise.resolve().then().catch` -- i.e. its object
    // chain nests the *prior* `.then()` call, which is the regression
    // this guards: a callback must capture its own call, not the chain
    // root's.
    let source = "Promise.resolve().then((value) => {}).catch((error) => {});\n";
    with_callback_args(source, |args| {
        let callees: Vec<&unsnarl_ir::reference::HeadExpression> =
            args.iter().flatten().map(|cb| &cb.callee).collect();
        assert_eq!(callees.len(), 2, "expected two annotated callbacks");

        // First callback (`.then`): `Promise.resolve().then`.
        let (then_obj, then_prop) =
            member_parts(callees[0]).expect("`.then`'s callee is a member access");
        assert_eq!(then_prop, "then");
        let resolve_call =
            call_callee(then_obj).expect("`.then`'s object is the `Promise.resolve()` call");
        let (promise, resolve_prop) =
            member_parts(resolve_call).expect("`Promise.resolve` is a member access");
        assert_eq!(resolve_prop, "resolve");
        assert_eq!(identifier_name(promise), Some("Promise"));

        // Second callback (`.catch`): `Promise.resolve().then().catch`.
        // The discriminating fact is that its object is the `.then(...)`
        // call, not the chain root.
        let (catch_obj, catch_prop) =
            member_parts(callees[1]).expect("`.catch`'s callee is a member access");
        assert_eq!(catch_prop, "catch");
        let then_call = call_callee(catch_obj).expect("`.catch`'s object is the `.then()` call");
        let (_, inner_then_prop) =
            member_parts(then_call).expect("`.catch`'s object chain ends at the prior `.then`");
        assert_eq!(
            inner_then_prop, "then",
            "each chained callback must capture its own call's callee"
        );
    });
}
