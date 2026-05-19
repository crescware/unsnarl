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
//! Context tracking:
//!
//! - `enter_node` / `leave_node` push and pop `path` (the
//!   `Vec<PathEntry>` consumed by `classify/*`). The `key` recorded on
//!   each path entry is whatever the parent's override pushed onto
//!   `key_stack` immediately before its `visit_*` call for this child.
//! - Each parent type whose field names matter to `classify` /
//!   `find_binding_root_context` overrides its `visit_*` and sets
//!   `key_stack` around each child visit. Types that the classify
//!   layer never observes (or that don't carry `IdentifierReference`
//!   children) inherit `walk_*` defaults.

use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentExpression, BlockStatement, CatchClause, Class,
    ExportSpecifier, ForInStatement, ForOfStatement, ForStatement, FormalParameter, Function,
    IdentifierReference, SwitchCase, SwitchStatement, UpdateExpression, VariableDeclarator,
};
use oxc_ast::AstKind;
use oxc_syntax::scope::ScopeFlags;

use unsnarl_oxc_parity::AstType;

use crate::enter_block::enter_block;
use crate::enter_catch::enter_catch;
use crate::enter_class::enter_class;
use crate::enter_for::{enter_for_in_statement, enter_for_of_statement, enter_for_statement};
use crate::enter_function::{enter_arrow_function_expression, enter_function};
use crate::enter_switch::enter_switch;
use crate::enter_switch_case::enter_switch_case;
use crate::handle_identifier_reference::handle_identifier_reference;
use crate::skip_block_scope::skip_block_scope;
use crate::state::{pop_scope, ScopeBuilderState};
use crate::walk::PathEntry;

pub(crate) struct ScopeBuildVisitor<'a, 'v> {
    pub(crate) state: &'v mut ScopeBuilderState,
    pub(crate) raw: &'v str,
    pub(crate) key_stack: Vec<Option<&'static str>>,
    pub(crate) path: Vec<PathEntry<'a>>,
}

impl<'a, 'v> ScopeBuildVisitor<'a, 'v> {
    pub(crate) fn new(state: &'v mut ScopeBuilderState, raw: &'v str) -> Self {
        Self {
            state,
            raw,
            key_stack: Vec::new(),
            path: Vec::new(),
        }
    }

    fn current_key(&self) -> Option<&'static str> {
        self.key_stack.last().copied().flatten()
    }

    fn parent_kind(&self) -> Option<AstKind<'a>> {
        // When a `visit_*` override fires, `enter_node` for the current
        // node has not yet pushed (it runs from inside the matching
        // `walk_*` body, which is the next step). The last entry on
        // `path` is therefore the immediate parent.
        self.path.last().map(|p| p.node)
    }
}

impl<'a, 'v> oxc_ast_visit::Visit<'a> for ScopeBuildVisitor<'a, 'v> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        let key = self.current_key();
        self.path.push(PathEntry { node: kind, key });
    }

    fn leave_node(&mut self, _kind: AstKind<'a>) {
        self.path.pop();
    }

    fn visit_block_statement(&mut self, it: &BlockStatement<'a>) {
        let skip = self
            .parent_kind()
            .as_ref()
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
        // Walk fields with key context so classify sees the right keys
        // for `params` (binding terminator) and `body`. We replicate
        // `walk_function`'s structure to keep `enter_node` /
        // `leave_node` firing in the right places.
        let kind = AstKind::Function(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        if let Some(id) = it.id.as_ref() {
            self.key_stack.push(Some("id"));
            self.visit_binding_identifier(id);
            self.key_stack.pop();
        }
        self.enter_scope(flags, &it.scope_id);
        if let Some(type_parameters) = it.type_parameters.as_deref() {
            self.visit_ts_type_parameter_declaration(type_parameters);
        }
        if let Some(this_param) = it.this_param.as_deref() {
            self.visit_ts_this_parameter(this_param);
        }
        self.key_stack.push(Some("params"));
        self.visit_formal_parameters(&it.params);
        self.key_stack.pop();
        if let Some(return_type) = it.return_type.as_deref() {
            self.visit_ts_type_annotation(return_type);
        }
        if let Some(body) = it.body.as_deref() {
            self.key_stack.push(Some("body"));
            self.visit_function_body(body);
            self.key_stack.pop();
        }
        self.leave_scope();
        self.leave_node(kind);
        pop_scope(self.state);
    }

    fn visit_arrow_function_expression(&mut self, it: &ArrowFunctionExpression<'a>) {
        enter_arrow_function_expression(self.state, it, self.raw);
        let kind = AstKind::ArrowFunctionExpression(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.enter_scope(
            {
                let mut flags = ScopeFlags::Function | ScopeFlags::Arrow;
                if it.has_use_strict_directive() {
                    flags |= ScopeFlags::StrictMode;
                }
                flags
            },
            &it.scope_id,
        );
        if let Some(type_parameters) = it.type_parameters.as_deref() {
            self.visit_ts_type_parameter_declaration(type_parameters);
        }
        self.key_stack.push(Some("params"));
        self.visit_formal_parameters(&it.params);
        self.key_stack.pop();
        if let Some(return_type) = it.return_type.as_deref() {
            self.visit_ts_type_annotation(return_type);
        }
        self.key_stack.push(Some("body"));
        self.visit_function_body(&it.body);
        self.key_stack.pop();
        self.leave_scope();
        self.leave_node(kind);
        pop_scope(self.state);
    }

    fn visit_formal_parameter(&mut self, it: &FormalParameter<'a>) {
        // Push `pattern` key so the binding terminator inside
        // `find_binding_root_context` recognises FormalParameter as
        // the param-binding root.
        let kind = AstKind::FormalParameter(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.visit_decorators(&it.decorators);
        self.key_stack.push(Some("pattern"));
        self.visit_binding_pattern(&it.pattern);
        self.key_stack.pop();
        if let Some(type_annotation) = it.type_annotation.as_deref() {
            self.visit_ts_type_annotation(type_annotation);
        }
        if let Some(initializer) = it.initializer.as_deref() {
            self.visit_expression(initializer);
        }
        self.leave_node(kind);
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

    fn visit_variable_declarator(&mut self, it: &VariableDeclarator<'a>) {
        let kind = AstKind::VariableDeclarator(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("id"));
        self.visit_binding_pattern(&it.id);
        self.key_stack.pop();
        if let Some(type_annotation) = it.type_annotation.as_deref() {
            self.visit_ts_type_annotation(type_annotation);
        }
        if let Some(init) = it.init.as_ref() {
            self.key_stack.push(Some("init"));
            self.visit_expression(init);
            self.key_stack.pop();
        }
        self.leave_node(kind);
    }

    fn visit_assignment_expression(&mut self, it: &AssignmentExpression<'a>) {
        let kind = AstKind::AssignmentExpression(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("left"));
        self.visit_assignment_target(&it.left);
        self.key_stack.pop();
        self.key_stack.push(Some("right"));
        self.visit_expression(&it.right);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_update_expression(&mut self, it: &UpdateExpression<'a>) {
        let kind = AstKind::UpdateExpression(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("argument"));
        self.visit_simple_assignment_target(&it.argument);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_export_specifier(&mut self, it: &ExportSpecifier<'a>) {
        let kind = AstKind::ExportSpecifier(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("local"));
        self.visit_module_export_name(&it.local);
        self.key_stack.pop();
        self.key_stack.push(Some("exported"));
        self.visit_module_export_name(&it.exported);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_identifier_reference(&mut self, it: &IdentifierReference<'a>) {
        // Capture parent / key BEFORE pushing IdentifierReference onto
        // the path: classify sees the identifier's parent as
        // `path.last()` and its key as the surrounding key_stack top.
        let parent = self.parent_kind();
        let key = self.current_key();
        handle_identifier_reference(
            self.state,
            parent.as_ref(),
            key,
            &self.path,
            it.name.as_str(),
            it.span,
            AstType::Identifier,
        );
        oxc_ast_visit::walk::walk_identifier_reference(self, it);
    }
}
