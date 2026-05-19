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

    let analyzed = run_analysis(&program, BoundarySourceType::Module, source);

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
