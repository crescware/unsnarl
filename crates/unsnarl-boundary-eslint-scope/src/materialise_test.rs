//! Sibling tests for `materialise.rs`.
//!
//! Covers the `_ => format!("{:?}", kind.ty()) → as_ast_type(...)`
//! fallback in `ast_type_of`. oxc nodes whose `AstType` name has no
//! counterpart in `unsnarl_oxc_parity::AstType` (e.g.
//! `PrivateInExpression`, `ArrayAssignmentTarget`,
//! `AssignmentTargetIdentifier`) collapse to `AstType::UnknownAstType`
//! when they appear on the walker's path. This is intentional but
//! lossy, so the test pins the behaviour: callbacks observing
//! `UnknownAstType` are looking at an oxc-only node that the boundary
//! layer hasn't (and may never) map onto an ESTree-style spelling.

use oxc_allocator::Allocator;

use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::ids::{ReferenceId, ScopeId};
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::Language;
use unsnarl_oxc_parity::AstType;

use crate::analyze::{analyze, AnalyzeOptions};
use crate::parser::{default_source_type_for, OxcParser, ParseOptions};
use crate::visitor::AnalysisVisitor;
use crate::ScopeBuilderState;

fn ast_type_label(ty: &AstType) -> &'static str {
    match ty {
        AstType::UnknownAstType => "UnknownAstType",
        AstType::Identifier => "Identifier",
        AstType::ExpressionStatement => "ExpressionStatement",
        AstType::Program => "Program",
        _ => "other",
    }
}

#[test]
fn private_in_expression_falls_back_to_unknown_ast_type_on_path() {
    // `#x in o` puts a `PrivateInExpression` on the walker's path
    // when the right-hand-side `IdentifierReference` (`o`) fires
    // `visit_identifier_reference`. `PrivateInExpression` has no
    // variant in `unsnarl_oxc_parity::AstType`, so the materialised
    // path must record it as `UnknownAstType`.
    struct Capture {
        saw_unknown_for_o: bool,
    }
    impl AnalysisVisitor for Capture {
        fn on_reference(
            &mut self,
            _ref_id: ReferenceId,
            _parent: Option<&AstNode>,
            _key: Option<&str>,
            path: &[AstNode],
            _scope_id: ScopeId,
            state: &ScopeBuilderState,
        ) {
            let identifier = &state.arena.references[_ref_id].identifier;
            if identifier.name() != "o" {
                return;
            }
            if path
                .iter()
                .any(|node| matches!(node.r#type, AstType::UnknownAstType))
            {
                self.saw_unknown_for_o = true;
            }
        }
    }
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
    let mut visitor = Capture {
        saw_unknown_for_o: false,
    };
    let _ = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            raw: parsed.raw,
        },
        &mut visitor,
    );
    assert!(
        visitor.saw_unknown_for_o,
        "`PrivateInExpression` on the path must materialise as UnknownAstType"
    );
}

#[test]
fn known_ast_type_on_path_does_not_collapse_to_unknown() {
    // Counter-example: an ordinary `obj.prop` path consists of
    // nodes (`ExpressionStatement`, `MemberExpression`, ...) whose
    // names are all present in `unsnarl_oxc_parity::AstType`, so
    // the materialised path must contain no `UnknownAstType`. This
    // guards the fallback against being entered accidentally for
    // the common case.
    struct Capture {
        any_unknown: bool,
    }
    impl AnalysisVisitor for Capture {
        fn on_reference(
            &mut self,
            _ref_id: ReferenceId,
            _parent: Option<&AstNode>,
            _key: Option<&str>,
            path: &[AstNode],
            _scope_id: ScopeId,
            _state: &ScopeBuilderState,
        ) {
            if path
                .iter()
                .any(|node| matches!(node.r#type, AstType::UnknownAstType))
            {
                self.any_unknown = true;
            }
        }
        fn on_diagnostic(&mut self, _diag: &Diagnostic) {}
    }
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
    let mut visitor = Capture { any_unknown: false };
    let _ = analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            raw: parsed.raw,
        },
        &mut visitor,
    );
    assert!(
        !visitor.any_unknown,
        "ordinary identifier paths must not hit the UnknownAstType fallback"
    );
    // The label helper is only used to keep the diagnostic readable
    // when the assertion above ever flips. Touch it here so it
    // doesn't get flagged as dead.
    let _ = ast_type_label(&AstType::UnknownAstType);
}
