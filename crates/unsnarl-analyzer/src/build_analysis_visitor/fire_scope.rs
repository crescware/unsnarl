//! `fire_scope`: fill the `ScopeAnnotation` row for a scope whose
//! block span matches an arena scope.

use oxc_ast::AstKind;
use oxc_span::Span;

use unsnarl_annotations::ScopeAnnotation;
use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};
use unsnarl_ir::primitive::Utf8ByteOffset;
use unsnarl_ir::scope::block_context::CaseClauseBlockContext;
use unsnarl_ir::scope::BlockContext;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_boundary::materialise::ast_node_of;
use unsnarl_oxc_parity::AstType;

use crate::block_context_of::block_context_of;
use crate::case_exits_function::case_exits_function;
use crate::case_falls_through::case_falls_through;
use crate::collect_abrupt_statements::collect_abrupt_statements;
use crate::format_case_test::format_case_test;

use super::BuildAnalysisVisitor;

const ZERO_DEPTHS: NestingDepths = NestingDepths {
    function: NestingDepth(0),
    r#if: NestingDepth(0),
    r#for: NestingDepth(0),
    r#while: NestingDepth(0),
    switch: NestingDepth(0),
    try_catch_finally: NestingDepth(0),
    block: NestingDepth(0),
};

impl<'a, 'arena> BuildAnalysisVisitor<'a, 'arena> {
    /// Fill the `ScopeAnnotation` row for a scope whose block matches
    /// `span`. Returns silently when `span` does not map to any scope
    /// (the BlockStatement under a CatchClause case).
    pub(super) fn fire_scope(&mut self, span: Span, kind: &AstKind<'a>) {
        let Some(&scope_id) = self.span_to_scope.get(&(span.start, span.end)) else {
            return;
        };
        let nesting_depths = self
            .nesting_depths
            .get(&span.start)
            .cloned()
            .unwrap_or_else(|| ZERO_DEPTHS.clone());
        let scope_type = self.arena.scopes[scope_id].r#type;
        let parent_node = self.parent_ast_node();
        let key = self.current_key();
        let is_switch_case = matches!(scope_type, ScopeType::Block)
            && matches!(ast_node_of(kind).r#type, AstType::SwitchCase);
        let (block_context, falls_through, exits_function) = if is_switch_case {
            let switch_case = match kind {
                AstKind::SwitchCase(c) => *c,
                _ => unreachable!("is_switch_case implies AstKind::SwitchCase"),
            };
            let case_test = switch_case
                .test
                .as_ref()
                .map(|expr| format_case_test(expr, self.index.raw()));
            let block_context = parent_node.zip(key).map(|(parent, k)| {
                let parent_offset = self.index.span_at(Utf8ByteOffset(parent.span.start)).offset;
                BlockContext::CaseClause(CaseClauseBlockContext::new(
                    parent.r#type.clone(),
                    k.to_string(),
                    parent_offset,
                    case_test,
                ))
            });
            let falls_through = case_falls_through(&switch_case.consequent);
            let exits_function = case_exits_function(&switch_case.consequent);
            (block_context, falls_through, exits_function)
        } else if matches!(scope_type, ScopeType::Function) {
            (None, false, false)
        } else {
            let block_context = block_context_of(parent_node, key, &self.path_entries, self.index);
            (block_context, false, false)
        };
        let callback_argument = self.callback_argument_for(scope_type, parent_node);
        let abrupt_statements = collect_abrupt_statements(kind, self.index);
        self.annotations.set_scope(
            scope_id,
            ScopeAnnotation {
                block_context,
                callback_argument,
                falls_through,
                exits_function,
                nesting_depths,
                abrupt_statements,
            },
        );
    }
}
