//! Per-offset nesting-depth snapshots.
//!
//! Walks the program AST, maintains seven counters (`Function`,
//! `If`, `For`, `While`, `Switch`, `TryCatchFinally`, `Block`), and
//! snapshots the current vector at each node's `start` offset.
//!
//! Block-statement classification depends on the immediate parent's
//! type and the slot key it occupies on that parent (e.g.
//! `IfStatement.consequent` increments `If`). To recover the
//! `(node, parent, key)` shape the visitor maintains a `key_stack`
//! alongside the ancestor stack and overrides every container
//! visitor that owns BlockStatement slots so it pushes / pops the
//! right key around each child visit.

use std::collections::HashMap;

use oxc_ast::ast::{
    CatchClause, ForInStatement, ForOfStatement, ForStatement, Function, IfStatement, Program,
    SwitchStatement, TryStatement, WhileStatement,
};
use oxc_ast::AstKind;
use oxc_ast_visit::Visit;
use oxc_syntax::scope::ScopeFlags;

use unsnarl_ir::nesting_kind::{NestingDepths, NestingKind};

mod counters;
mod visitor_helpers;

use counters::Counters;

pub fn compute_nesting_depths(program: &Program<'_>) -> HashMap<u32, NestingDepths> {
    let mut visitor = NestingDepthVisitor::new();
    visitor.visit_program(program);
    visitor.depths_by_offset
}

struct NestingDepthVisitor {
    counters: Counters,
    depths_by_offset: HashMap<u32, NestingDepths>,
    /// Each entry corresponds to one `enter_node` call. `Some(kind)`
    /// means we incremented `kind`'s counter at that entry; `None`
    /// means we entered the node without an increment.
    enter_stack: Vec<Option<NestingKind>>,
    /// Type of the immediate parent during a child visit. Pushed in
    /// `enter_node`, popped in `leave_node`.
    parent_types: Vec<ParentKind>,
    /// The key slot the next child visit occupies on its parent.
    /// Container visitors push/pop around each visit_* call; bare
    /// (un-overridden) walks see `None`.
    key_stack: Vec<Option<&'static str>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ParentKind {
    Function,
    Arrow,
    If,
    For,
    ForIn,
    ForOf,
    While,
    DoWhile,
    Try,
    Catch,
    Other,
}

impl<'a> Visit<'a> for NestingDepthVisitor {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        let mut inc: Option<NestingKind> = None;
        match &kind {
            AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) => {
                self.counters.inc(NestingKind::Function);
                inc = Some(NestingKind::Function);
            }
            AstKind::BlockStatement(_) => {
                if let Some(kind) = self.classify_block() {
                    self.counters.inc(kind);
                    inc = Some(kind);
                }
            }
            AstKind::SwitchStatement(_) => {
                self.counters.inc(NestingKind::Switch);
                inc = Some(NestingKind::Switch);
            }
            _ => {}
        }
        let span = match &kind {
            AstKind::Program(p) => p.span,
            AstKind::BlockStatement(s) => s.span,
            AstKind::Function(f) => f.span,
            AstKind::ArrowFunctionExpression(f) => f.span,
            AstKind::IfStatement(s) => s.span,
            AstKind::ForStatement(s) => s.span,
            AstKind::ForInStatement(s) => s.span,
            AstKind::ForOfStatement(s) => s.span,
            AstKind::WhileStatement(s) => s.span,
            AstKind::DoWhileStatement(s) => s.span,
            AstKind::SwitchStatement(s) => s.span,
            AstKind::SwitchCase(c) => c.span,
            AstKind::TryStatement(s) => s.span,
            AstKind::CatchClause(c) => c.span,
            AstKind::Class(c) => c.span,
            _ => {
                self.enter_stack.push(inc);
                self.parent_types.push(Self::parent_kind_of(&kind));
                return;
            }
        };
        self.depths_by_offset
            .insert(span.start, self.counters.snapshot());
        self.enter_stack.push(inc);
        self.parent_types.push(Self::parent_kind_of(&kind));
    }

    fn leave_node(&mut self, _kind: AstKind<'a>) {
        self.parent_types.pop();
        if let Some(Some(k)) = self.enter_stack.pop() {
            self.counters.dec(k);
        }
    }

    fn visit_if_statement(&mut self, it: &IfStatement<'a>) {
        let kind = AstKind::IfStatement(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("test"));
        self.visit_expression(&it.test);
        self.key_stack.pop();
        self.key_stack.push(Some("consequent"));
        self.visit_statement(&it.consequent);
        self.key_stack.pop();
        if let Some(alt) = &it.alternate {
            self.key_stack.push(Some("alternate"));
            self.visit_statement(alt);
            self.key_stack.pop();
        }
        self.leave_node(kind);
    }

    fn visit_for_statement(&mut self, it: &ForStatement<'a>) {
        let kind = AstKind::ForStatement(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope(ScopeFlags::empty(), &it.scope_id);
        self.visit_span(&it.span);
        if let Some(init) = &it.init {
            self.key_stack.push(Some("init"));
            self.visit_for_statement_init(init);
            self.key_stack.pop();
        }
        if let Some(test) = &it.test {
            self.key_stack.push(Some("test"));
            self.visit_expression(test);
            self.key_stack.pop();
        }
        if let Some(update) = &it.update {
            self.key_stack.push(Some("update"));
            self.visit_expression(update);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("body"));
        self.visit_statement(&it.body);
        self.key_stack.pop();
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_for_in_statement(&mut self, it: &ForInStatement<'a>) {
        let kind = AstKind::ForInStatement(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope(ScopeFlags::empty(), &it.scope_id);
        self.visit_span(&it.span);
        self.key_stack.push(Some("left"));
        self.visit_for_statement_left(&it.left);
        self.key_stack.pop();
        self.key_stack.push(Some("right"));
        self.visit_expression(&it.right);
        self.key_stack.pop();
        self.key_stack.push(Some("body"));
        self.visit_statement(&it.body);
        self.key_stack.pop();
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_for_of_statement(&mut self, it: &ForOfStatement<'a>) {
        let kind = AstKind::ForOfStatement(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope(ScopeFlags::empty(), &it.scope_id);
        self.visit_span(&it.span);
        self.key_stack.push(Some("left"));
        self.visit_for_statement_left(&it.left);
        self.key_stack.pop();
        self.key_stack.push(Some("right"));
        self.visit_expression(&it.right);
        self.key_stack.pop();
        self.key_stack.push(Some("body"));
        self.visit_statement(&it.body);
        self.key_stack.pop();
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_while_statement(&mut self, it: &WhileStatement<'a>) {
        let kind = AstKind::WhileStatement(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("test"));
        self.visit_expression(&it.test);
        self.key_stack.pop();
        self.key_stack.push(Some("body"));
        self.visit_statement(&it.body);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_do_while_statement(&mut self, it: &oxc_ast::ast::DoWhileStatement<'a>) {
        let kind = AstKind::DoWhileStatement(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("body"));
        self.visit_statement(&it.body);
        self.key_stack.pop();
        self.key_stack.push(Some("test"));
        self.visit_expression(&it.test);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_try_statement(&mut self, it: &TryStatement<'a>) {
        let kind = AstKind::TryStatement(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("block"));
        self.visit_block_statement(&it.block);
        self.key_stack.pop();
        if let Some(handler) = &it.handler {
            // CatchClause has no key on its parent; the BlockStatement
            // under it is reached via CatchClause's own visit override.
            self.visit_catch_clause(handler);
        }
        if let Some(finalizer) = &it.finalizer {
            self.key_stack.push(Some("finalizer"));
            self.visit_block_statement(finalizer);
            self.key_stack.pop();
        }
        self.leave_node(kind);
    }

    fn visit_catch_clause(&mut self, it: &CatchClause<'a>) {
        let kind = AstKind::CatchClause(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope(ScopeFlags::CatchClause, &it.scope_id);
        self.visit_span(&it.span);
        if let Some(param) = &it.param {
            self.visit_catch_parameter(param);
        }
        self.key_stack.push(Some("body"));
        self.visit_block_statement(&it.body);
        self.key_stack.pop();
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_switch_statement(&mut self, it: &SwitchStatement<'a>) {
        let kind = AstKind::SwitchStatement(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope(ScopeFlags::empty(), &it.scope_id);
        self.visit_span(&it.span);
        self.key_stack.push(Some("discriminant"));
        self.visit_expression(&it.discriminant);
        self.key_stack.pop();
        for case in &it.cases {
            self.visit_switch_case(case);
        }
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_function(&mut self, it: &Function<'a>, flags: ScopeFlags) {
        let kind = AstKind::Function(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope(flags | ScopeFlags::Function, &it.scope_id);
        self.visit_span(&it.span);
        if let Some(id) = &it.id {
            self.visit_binding_identifier(id);
        }
        if let Some(type_params) = &it.type_parameters {
            self.visit_ts_type_parameter_declaration(type_params);
        }
        if let Some(this_param) = &it.this_param {
            self.visit_ts_this_parameter(this_param);
        }
        self.visit_formal_parameters(&it.params);
        if let Some(return_type) = &it.return_type {
            self.visit_ts_type_annotation(return_type);
        }
        if let Some(body) = &it.body {
            self.key_stack.push(Some("body"));
            self.visit_function_body(body);
            self.key_stack.pop();
        }
        self.leave_scope();
        self.leave_node(kind);
    }

    fn visit_arrow_function_expression(&mut self, it: &oxc_ast::ast::ArrowFunctionExpression<'a>) {
        let kind = AstKind::ArrowFunctionExpression(self.alloc(it));
        self.enter_node(kind);
        self.enter_scope(ScopeFlags::Function | ScopeFlags::Arrow, &it.scope_id);
        self.visit_span(&it.span);
        if let Some(type_params) = &it.type_parameters {
            self.visit_ts_type_parameter_declaration(type_params);
        }
        self.visit_formal_parameters(&it.params);
        if let Some(return_type) = &it.return_type {
            self.visit_ts_type_annotation(return_type);
        }
        self.key_stack.push(Some("body"));
        self.visit_function_body(&it.body);
        self.key_stack.pop();
        self.leave_scope();
        self.leave_node(kind);
    }
}

#[cfg(test)]
#[path = "compute_nesting_depths_test.rs"]
mod compute_nesting_depths_test;
