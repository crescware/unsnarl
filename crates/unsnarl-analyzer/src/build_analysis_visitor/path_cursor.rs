//! Path / cursor bookkeeping: maintain the `path` + `path_entries`
//! stacks and expose the current slot key / arg index / parent node.

use oxc_ast::AstKind;

use unsnarl_ir::primitive::AstNode;
use unsnarl_oxc_boundary::materialise::ast_node_of;

use crate::path_entry::{ArrowBodyInfo, PathEntry};

use super::{BuildAnalysisVisitor, PathFrame};

impl<'a, 'arena> BuildAnalysisVisitor<'a, 'arena> {
    pub(super) fn ast_node_of_kind(&self, kind: &AstKind<'a>) -> AstNode {
        let mut node = ast_node_of(kind);
        if matches!(kind, AstKind::Program(_)) {
            node.span = oxc_span::Span::new(self.program_normalised_start, node.span.end);
        }
        node
    }

    pub(super) fn current_key(&self) -> Option<&'static str> {
        self.key_stack.last().copied().flatten()
    }

    pub(super) fn current_arg_index(&self) -> Option<usize> {
        self.arg_index_stack.last().copied().flatten()
    }

    pub(super) fn parent_ast_node(&self) -> Option<&AstNode> {
        self.path_entries.last().map(|e| &e.node)
    }

    pub(super) fn push_path(&mut self, kind: AstKind<'a>, arrow_body: Option<ArrowBodyInfo>) {
        let key = self.current_key();
        let node = self.ast_node_of_kind(&kind);
        self.path.push(PathFrame { kind, arrow_body });
        self.path_entries.push(match arrow_body {
            None => PathEntry::new(node, key),
            Some(ab) => PathEntry::with_arrow_body(node, key, ab.span, ab.is_block),
        });
    }

    pub(super) fn pop_path(&mut self) {
        self.path.pop();
        self.path_entries.pop();
    }
}
