//! Walker that drives the eslint-scope-compatible scope build.
//!
//! Mirrors the TS `walk(ast, { enter, leave })` invocation in
//! `analyze.ts` plus the `handleEnter` / `handleLeave` dispatchers.
//! TS keeps `walk` separate from the dispatchers because it builds the
//! traversal on top of a generic `WalkVisitor` callback shape. Rust
//! collapses both into a single `ScopeBuildVisitor` that implements
//! `oxc_ast_visit::Visit<'a>` directly: each per-AST-shape `visit_*`
//! override plays the role of the TS dispatcher's `case` arm.
//!
//! Parent-context tracking (`parent_stack`) is maintained through
//! `enter_node` / `leave_node`. When a `visit_*` override fires for
//! some node, `enter_node` has already pushed every ancestor but has
//! not yet pushed the node itself, so `parent_stack.last()` is the
//! immediate parent — exactly the value the TS `handleEnter` reads
//! out of its `parent` argument.

use oxc_ast::ast::BlockStatement;
use oxc_ast::AstKind;

use crate::enter_block::enter_block;
use crate::skip_block_scope::skip_block_scope;
use crate::state::{pop_scope, ScopeBuilderState};

pub(crate) struct ScopeBuildVisitor<'a, 'v> {
    pub(crate) state: &'v mut ScopeBuilderState,
    pub(crate) raw: &'v str,
    pub(crate) parent_stack: Vec<AstKind<'a>>,
}

impl<'a, 'v> ScopeBuildVisitor<'a, 'v> {
    pub(crate) fn new(state: &'v mut ScopeBuilderState, raw: &'v str) -> Self {
        Self {
            state,
            raw,
            parent_stack: Vec::new(),
        }
    }
}

impl<'a, 'v> oxc_ast_visit::Visit<'a> for ScopeBuildVisitor<'a, 'v> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        self.parent_stack.push(kind);
    }

    fn leave_node(&mut self, _kind: AstKind<'a>) {
        self.parent_stack.pop();
    }

    fn visit_block_statement(&mut self, it: &BlockStatement<'a>) {
        let skip = self
            .parent_stack
            .last()
            .map(skip_block_scope)
            .unwrap_or(false);
        if skip {
            oxc_ast_visit::walk::walk_block_statement(self, it);
            return;
        }
        enter_block(self.state, it, self.raw);
        oxc_ast_visit::walk::walk_block_statement(self, it);
        pop_scope(self.state);
    }
}
