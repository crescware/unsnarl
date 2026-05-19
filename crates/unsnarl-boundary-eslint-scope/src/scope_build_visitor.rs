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

use oxc_ast::ast::{
    ArrowFunctionExpression, BlockStatement, CatchClause, Class, ForInStatement, ForOfStatement,
    ForStatement, Function, SwitchCase, SwitchStatement,
};
use oxc_ast::AstKind;
use oxc_syntax::scope::ScopeFlags;

use crate::enter_block::enter_block;
use crate::enter_catch::enter_catch;
use crate::enter_class::enter_class;
use crate::enter_for::{enter_for_in_statement, enter_for_of_statement, enter_for_statement};
use crate::enter_function::{enter_arrow_function_expression, enter_function};
use crate::enter_switch::enter_switch;
use crate::enter_switch_case::enter_switch_case;
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

    fn visit_function(&mut self, it: &Function<'a>, flags: ScopeFlags) {
        enter_function(self.state, it, self.raw);
        oxc_ast_visit::walk::walk_function(self, it, flags);
        pop_scope(self.state);
    }

    fn visit_arrow_function_expression(&mut self, it: &ArrowFunctionExpression<'a>) {
        enter_arrow_function_expression(self.state, it, self.raw);
        oxc_ast_visit::walk::walk_arrow_function_expression(self, it);
        pop_scope(self.state);
    }

    fn visit_class(&mut self, it: &Class<'a>) {
        enter_class(self.state, it);
        oxc_ast_visit::walk::walk_class(self, it);
        pop_scope(self.state);
    }

    fn visit_catch_clause(&mut self, it: &CatchClause<'a>) {
        enter_catch(self.state, it, self.raw);
        oxc_ast_visit::walk::walk_catch_clause(self, it);
        pop_scope(self.state);
    }

    fn visit_for_statement(&mut self, it: &ForStatement<'a>) {
        enter_for_statement(self.state, it, self.raw);
        oxc_ast_visit::walk::walk_for_statement(self, it);
        pop_scope(self.state);
    }

    fn visit_for_in_statement(&mut self, it: &ForInStatement<'a>) {
        enter_for_in_statement(self.state, it, self.raw);
        oxc_ast_visit::walk::walk_for_in_statement(self, it);
        pop_scope(self.state);
    }

    fn visit_for_of_statement(&mut self, it: &ForOfStatement<'a>) {
        enter_for_of_statement(self.state, it, self.raw);
        oxc_ast_visit::walk::walk_for_of_statement(self, it);
        pop_scope(self.state);
    }

    fn visit_switch_statement(&mut self, it: &SwitchStatement<'a>) {
        enter_switch(self.state, it);
        oxc_ast_visit::walk::walk_switch_statement(self, it);
        pop_scope(self.state);
    }

    fn visit_switch_case(&mut self, it: &SwitchCase<'a>) {
        enter_switch_case(self.state, it, self.raw);
        oxc_ast_visit::walk::walk_switch_case(self, it);
        pop_scope(self.state);
    }
}
