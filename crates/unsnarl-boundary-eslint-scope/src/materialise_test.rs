//! Sibling tests for `materialise.rs`.
//!
//! Covers the `_ => format!("{:?}", kind.ty()) → as_ast_type(...)`
//! fallback in `ast_type_of`. oxc nodes whose `AstType` name has no
//! counterpart in `unsnarl_oxc_parity::AstType` (e.g.
//! `PrivateInExpression`, `ArrayAssignmentTarget`,
//! `AssignmentTargetIdentifier`) collapse to `AstType::UnknownAstType`
//! when their kind is materialised. This is intentional but lossy, so
//! the test pins the behaviour: callers observing `UnknownAstType` are
//! looking at an oxc-only node that the boundary layer hasn't (and may
//! never) map onto an ESTree-style spelling.

use oxc_allocator::Allocator;
use oxc_ast::AstKind;
use oxc_semantic::SemanticBuilder;

use unsnarl_ir::Language;
use unsnarl_oxc_parity::AstType;

use crate::materialise::ast_type_of;
use crate::parser::{default_source_type_for, OxcParser, ParseOptions};

#[test]
fn private_in_expression_falls_back_to_unknown_ast_type() {
    let allocator = Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            "class C { #x = 1; has(o: any) { return #x in o; } }\n",
            &ParseOptions {
                language: Language::Ts,
                source_path: "input.ts".to_string(),
                source_type: default_source_type_for(Language::Ts),
            },
        )
        .expect("must parse");
    let semantic = SemanticBuilder::new().build(&parsed.program).semantic;
    let nodes = semantic.nodes();
    let saw_unknown = nodes.iter().any(|node| {
        matches!(node.kind(), AstKind::PrivateInExpression(_))
            && matches!(ast_type_of(&node.kind()), AstType::UnknownAstType)
    });
    assert!(
        saw_unknown,
        "`PrivateInExpression` must materialise as UnknownAstType",
    );
}

#[test]
fn known_ast_type_does_not_collapse_to_unknown() {
    // Counter-example: an ordinary `obj.prop` AST consists of nodes
    // (`ExpressionStatement`, `MemberExpression`, ...) whose names
    // are all present in `unsnarl_oxc_parity::AstType`, so none of
    // them collapse to `UnknownAstType`. Guards the fallback against
    // being entered accidentally for the common case.
    let allocator = Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            "obj.prop;\n",
            &ParseOptions {
                language: Language::Ts,
                source_path: "input.ts".to_string(),
                source_type: default_source_type_for(Language::Ts),
            },
        )
        .expect("must parse");
    let semantic = SemanticBuilder::new().build(&parsed.program).semantic;
    let any_unknown = semantic
        .nodes()
        .iter()
        .any(|node| matches!(ast_type_of(&node.kind()), AstType::UnknownAstType));
    assert!(
        !any_unknown,
        "ordinary identifier nodes must not hit the UnknownAstType fallback",
    );
}
