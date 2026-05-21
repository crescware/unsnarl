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
