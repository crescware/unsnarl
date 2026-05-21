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
use unsnarl_boundary_eslint_scope::parser::SourceType as BoundarySourceType;

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
    // fires on the TS side (inline with scope-build), only the
    // hoistings encountered SO FAR are visible, so TS resolves to the
    // outer (implicit-global) binding instead. The Rust analyzer must
    // mirror what TS sees at reference-binding time, which is exactly
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
    // typically an enclosing IfStatement). The TS AST spells this
    // slot `"body"` and the IR must match.
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
        "the block's blockContext.key must mirror the TS AST slot \
         label `body`, not the auto-walker's `consequent` carryover"
    );
}

#[test]
fn class_inside_sequence_expression_reports_expressions_slot_key() {
    // Parity regression: when a ClassExpression participates in a
    // comma-separated SequenceExpression, the class scope's
    // `blockContext.key` used to come out as `"argument"` (carried
    // over from an enclosing argument-shaped parent in the walker's
    // key stack). The TS AST spells the slot `"expressions"`.
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
        "the class's blockContext.key must mirror the TS AST slot \
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
    // does not push a per-child key. The TS AST spells this slot
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
        "the class's blockContext.key must mirror the TS AST slot \
         label `declaration`, not the auto-walker's `body` carryover"
    );
}

#[test]
fn export_named_with_from_creates_implicit_global_reference_for_local_name() {
    // Parity regression: `export { Lexer } from './Lexer.js'`
    // surfaces in npm `oxc-parser` as `ExportSpecifier.local.type =
    // "Identifier"`, and the TS pipeline routes that identifier
    // through `handleIdentifierReference`, producing a read
    // reference that resolves to an implicit-global `Lexer` in the
    // module scope. The Rust `oxc_parser` crate keeps the same slot
    // as `ModuleExportName::IdentifierName` for re-exports (and
    // anywhere the local name is a reserved word like `default`),
    // so the analyzer used to walk past it without firing the
    // reference. The boundary's `visit_identifier_name` now routes
    // these slots through `handle_identifier_reference` so the IR
    // stays byte-identical to TS.
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
    // Parity regression: `export * as default from './base.js'`
    // surfaces in npm `oxc-parser` as
    // `ExportAllDeclaration.exported.type = "Identifier"`, and the
    // TS pipeline routes that identifier through
    // `handleIdentifierReference` -- producing an implicit-global
    // `default` plus one read reference. The Rust `oxc_parser` crate
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
    // Parity regression: `this.#prop = rhs` (and `obj.#prop = rhs`)
    // surfaced in npm `oxc-parser` as `MemberExpression(object,
    // PrivateIdentifier("prop"))` with `computed: false`. The TS
    // head-builder reads `property.name` directly without checking
    // the property's node type, so the resulting
    // `expressionStatementContainer.head` is
    // `{ kind: assign, left: { kind: member, property: "prop" }, ... }`.
    // The Rust `oxc_parser` crate keeps PrivateFieldExpression
    // separate from StaticMemberExpression in three head-building
    // arms (`Expression`, `AssignmentTarget`,
    // `SimpleAssignmentTarget`); each of those used to bail to
    // `None`, which then collapsed the whole assignment head to
    // `kind: raw`. Mirror the TS flattening.
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
        "the private field name must be flattened to the bare identifier (no leading `#`) to match TS"
    );
}
