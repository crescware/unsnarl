//! Build the `AstNode` recorded on a scope's `block` field.

use oxc_ast::AstKind;
use oxc_span::{GetSpan, Span};

use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::Language;

use crate::materialise::ast_node_of;

/// Build the `AstNode` recorded on a scope's `block` field, applying
/// the TypeScript-only `Program` span normalisation when the scope's
/// anchor is the root `Program`.
///
/// Background: npm `oxc-parser` exposes `program.start = 0` for
/// `lang: "js" / "jsx"`, but for `lang: "ts" / "tsx"` it advances
/// past any leading hashbang and leading line / block comments so
/// `program.start` lands on the first directive / body statement.
/// The Rust `oxc_parser` crate emits `Program.span.start = 0`
/// unconditionally, so the adapter normalises the start here so the
/// root `block.span` matches the parity baseline for TypeScript
/// inputs whose source begins with comments / a hashbang (e.g.
/// cytoscape.min.js).
pub(super) fn build_anchor_node(kind: &AstKind<'_>, language: Language) -> AstNode {
    let mut node = ast_node_of(kind);
    if matches!(kind, AstKind::Program(_)) && matches!(language, Language::Ts | Language::Tsx) {
        if let AstKind::Program(program) = kind {
            let normalised_start = program
                .directives
                .first()
                .map(|d| d.span.start)
                .or_else(|| program.body.first().map(|s| s.span().start))
                .or_else(|| program.hashbang.as_ref().map(|h| h.span.end))
                .unwrap_or(program.span.start);
            node.span = Span::new(normalised_start, program.span.end);
        }
    }
    node
}
