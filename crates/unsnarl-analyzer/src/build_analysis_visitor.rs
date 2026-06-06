//! Walker that fills the per-entity side-table annotations after the
//! scope build.
//!
//! Runs a separate `oxc_ast_visit::Visit` pass after `analyze`
//! returns because several analyzer functions need full `AstKind`
//! handles (`expression_statement_container`, `find_reference_owners`,
//! `case_falls_through`, `case_exits_function`, `format_case_test`) or
//! per-entry keys (`find_predicate_container`, `if_chain_root_offset`).
//!
//! The walker is keyed against the arena built by the boundary: scope
//! rows are looked up by `scope.block.span`, reference rows by
//! `reference.identifier.span`. Encountered AST nodes whose spans do
//! not match any arena row are simply skipped (e.g. a `BlockStatement`
//! that is the body of a `CatchClause`, which the boundary merges into
//! the enclosing catch scope rather than allocating a new one).

use std::collections::HashMap;

use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentExpression, BindingIdentifier, BlockStatement,
    CallExpression, CatchClause, Class, ComputedMemberExpression, DoWhileStatement,
    ExportDefaultDeclaration, ExportNamedDeclaration, ExpressionStatement, ForInStatement,
    ForOfStatement, ForStatement, Function, IdentifierName, IdentifierReference, IfStatement,
    ImportAttribute, JSXIdentifier, LabeledStatement, MetaProperty, NewExpression, ObjectProperty,
    PrivateFieldExpression, Program, ReturnStatement, SequenceExpression, StaticMemberExpression,
    SwitchCase, SwitchStatement, ThrowStatement, TryStatement, UpdateExpression,
    VariableDeclarator, WhileStatement,
};
use oxc_ast::AstKind;
use oxc_ast_visit::Visit;
use oxc_syntax::scope::ScopeFlags;

use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::{IrArena, ReferenceId, ScopeId};

use crate::annotations_impl::AnnotationsImpl;
use crate::path_entry::{ArrowBodyInfo, PathEntry};

mod callback_host;
mod fire_reference;
mod fire_scope;
mod is_explicitly_handled;
mod path_cursor;

use is_explicitly_handled::is_explicitly_handled;

/// Internal walk-time frame: the live `AstKind` handle plus, for
/// `ArrowFunctionExpression`, the body-shape side-channel
/// `find_completion` needs to distinguish expression-body arrows from
/// block-body arrows.
///
/// The slot key on the parent and the materialised `AstNode` are held
/// on the parallel `path_entries: Vec<PathEntry>` so the analyzer
/// helpers can borrow them as `&[PathEntry]` without re-materialising
/// per fire_* call.
struct PathFrame<'a> {
    kind: AstKind<'a>,
    arrow_body: Option<ArrowBodyInfo>,
}

pub(crate) struct BuildAnalysisVisitor<'a, 'arena> {
    index: &'arena SourceIndex<'arena>,
    arena: &'arena IrArena,
    annotations: &'arena mut AnnotationsImpl,
    nesting_depths: &'arena HashMap<u32, NestingDepths>,
    span_to_scope: &'arena HashMap<(u32, u32), ScopeId>,
    span_to_ref: &'arena HashMap<(u32, u32), ReferenceId>,
    key_stack: Vec<Option<&'static str>>,
    /// Pushed *only* when entering an argument slot of a
    /// `CallExpression` / `NewExpression`; popped on the way out.
    /// Consequently the stack is **not** strictly parallel with
    /// `key_stack` -- visiting the call's `callee` /
    /// `type_arguments` does not push a `None` placeholder, so a
    /// raw `last()` would leak the enclosing arg's index into the
    /// callee subtree. `callback_argument_for` defends against that
    /// by additionally checking `current_key() == Some("arguments")`
    /// before reading the top. Kept separate from `key_stack` so
    /// existing `&'static str` keys keep their lifetime guarantees.
    arg_index_stack: Vec<Option<usize>>,
    path: Vec<PathFrame<'a>>,
    /// Parallel to `path`, kept in lock-step on push / pop. Holds
    /// the lifetime-free `(AstNode, key, arrow_body)` triple that the
    /// path-walking analyzer helpers consume. Materialised once per
    /// node entry instead of being re-cloned from `path` on every
    /// `fire_scope` / `fire_reference` call — on minified bundles
    /// (mermaid.js: ~250k fire_* calls, depth 20+) that copy is the
    /// single hottest source of allocator churn and per-ancestor
    /// `ast_type_of` work.
    path_entries: Vec<PathEntry>,
    /// Normalised `Program.span.start` matching the boundary's
    /// hashbang/directive/body offset. Used when materialising
    /// `AstNode { Program, span }` so downstream consumers
    /// (`block_context_of`, `find_predicate_container`, ...) see
    /// the same start offset the boundary stamped onto the global
    /// scope's `block`. See `analyze::analyze` for the source.
    program_normalised_start: u32,
}

impl<'a, 'arena> BuildAnalysisVisitor<'a, 'arena> {
    pub(crate) fn new(
        index: &'arena SourceIndex<'arena>,
        arena: &'arena IrArena,
        annotations: &'arena mut AnnotationsImpl,
        nesting_depths: &'arena HashMap<u32, NestingDepths>,
        span_to_scope: &'arena HashMap<(u32, u32), ScopeId>,
        span_to_ref: &'arena HashMap<(u32, u32), ReferenceId>,
        program_normalised_start: u32,
    ) -> Self {
        Self {
            index,
            arena,
            annotations,
            nesting_depths,
            span_to_scope,
            span_to_ref,
            key_stack: Vec::new(),
            arg_index_stack: Vec::new(),
            path: Vec::new(),
            path_entries: Vec::new(),
            program_normalised_start,
        }
    }
}

impl<'a, 'arena> Visit<'a> for BuildAnalysisVisitor<'a, 'arena> {
    fn visit_program(&mut self, it: &Program<'a>) {
        // The global / module scope must not carry a `ScopeAnnotation`:
        // `on_scope` is only invoked from the boundary's per-block
        // `enter_*` helpers, and the global scope is constructed by
        // the scope manager without going through any of them.
        // Consumers read the default zero-valued annotation back via
        // `Annotations::of_scope` (the same shape
        // `AnnotationsImpl::empty_scope_annotation` returns here).
        // Walk the program tree to populate child scopes and reference
        // rows, but do NOT call `fire_scope` on the Program node.
        let kind = AstKind::Program(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        if let Some(hashbang) = it.hashbang.as_ref() {
            self.visit_hashbang(hashbang);
        }
        self.visit_directives(&it.directives);
        self.key_stack.push(Some("body"));
        self.visit_statements(&it.body);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_block_statement(&mut self, it: &BlockStatement<'a>) {
        let kind = AstKind::BlockStatement(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
        self.key_stack.push(Some("body"));
        oxc_ast_visit::walk::walk_block_statement(self, it);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_function(&mut self, it: &Function<'a>, _flags: ScopeFlags) {
        let kind = AstKind::Function(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
        self.visit_span(&it.span);
        if let Some(id) = it.id.as_ref() {
            self.key_stack.push(Some("id"));
            self.visit_binding_identifier(id);
            self.key_stack.pop();
        }
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
        self.pop_path();
    }

    fn visit_arrow_function_expression(&mut self, it: &ArrowFunctionExpression<'a>) {
        let kind = AstKind::ArrowFunctionExpression(self.alloc(it));
        let arrow_body = Some(ArrowBodyInfo {
            span: it.body.span,
            is_block: !it.expression,
        });
        self.fire_scope(it.span, &kind);
        self.push_path(kind, arrow_body);
        self.visit_span(&it.span);
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
        if it.expression {
            // Expression-body arrow: oxc wraps the expression in a
            // synthetic `FunctionBody { [ExpressionStatement(expr)] }`,
            // but the IR expects the ESTree shape
            // `ArrowFunctionExpression.body: Expression`. Walk the
            // inner expression directly so subsequent scope /
            // reference rows see `parent = ArrowFunctionExpression,
            // key = "body"` instead of inheriting the synthetic
            // `ExpressionStatement.expression` slot.
            if let Some(oxc_ast::ast::Statement::ExpressionStatement(es)) =
                it.body.statements.first()
            {
                self.visit_expression(&es.expression);
            } else {
                self.visit_function_body(&it.body);
            }
        } else {
            self.visit_function_body(&it.body);
        }
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_class(&mut self, it: &Class<'a>) {
        let kind = AstKind::Class(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
        oxc_ast_visit::walk::walk_class(self, it);
        self.pop_path();
    }

    fn visit_export_named_declaration(&mut self, it: &ExportNamedDeclaration<'a>) {
        // `block_context_of` keys the emitted `blockContext.key` off
        // the current slot label. Without this override the inner
        // declaration (class / function / variable) inherits whatever
        // the surrounding statement list pushed -- typically `"body"`
        // from `Program.body` -- and would surface in the IR as
        // `{ parentType: "ExportNamedDeclaration", key: "body" }`
        // instead of the expected `key: "declaration"`. Push the
        // ESTree visitorKey list
        // `["declaration", "specifiers", "source", "attributes"]`.
        let kind = AstKind::ExportNamedDeclaration(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        if let Some(declaration) = &it.declaration {
            self.key_stack.push(Some("declaration"));
            self.visit_declaration(declaration);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("specifiers"));
        self.visit_export_specifiers(&it.specifiers);
        self.key_stack.pop();
        if let Some(source) = &it.source {
            self.key_stack.push(Some("source"));
            self.visit_string_literal(source);
            self.key_stack.pop();
        }
        if let Some(with_clause) = &it.with_clause {
            self.key_stack.push(Some("attributes"));
            self.visit_with_clause(with_clause);
            self.key_stack.pop();
        }
        self.pop_path();
    }

    fn visit_object_property(&mut self, it: &ObjectProperty<'a>) {
        // Same family as `visit_export_named_declaration`: oxc's
        // auto-generated walker walks the inner `key` / `value`
        // slots without pushing their per-child key onto
        // `key_stack`, so a function / class expression that lands in
        // `value` (e.g. `{ key: function () {} }` inside a call
        // argument) inherits whatever surrounding label was in scope
        // -- frequently `"arguments"` from an enclosing
        // `CallExpression.arguments` -- and would surface in the IR as
        // `{ parentType: "Property", key: "arguments" }` instead of
        // the expected `key: "value"`.
        let kind = AstKind::ObjectProperty(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("key"));
        self.visit_property_key(&it.key);
        self.key_stack.pop();
        self.key_stack.push(Some("value"));
        self.visit_expression(&it.value);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_export_default_declaration(&mut self, it: &ExportDefaultDeclaration<'a>) {
        // Same family as `visit_export_named_declaration`: oxc's
        // auto-generated walker visits the inner declaration without
        // pushing the slot label `"declaration"` onto `key_stack`, so
        // the declaration's child scope (a function / class scope)
        // inherits `"body"` from the surrounding `Program.body` slot
        // and would surface in the IR as
        // `{ parentType: "ExportDefaultDeclaration", key: "body" }`
        // instead of the expected `key: "declaration"`. The
        // `exported` field is metadata only (always the literal name
        // "default" for a default export) and is not in the ESTree
        // visitorKey list either.
        let kind = AstKind::ExportDefaultDeclaration(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("declaration"));
        self.visit_export_default_declaration_kind(&it.declaration);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_catch_clause(&mut self, it: &CatchClause<'a>) {
        let kind = AstKind::CatchClause(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
        self.visit_span(&it.span);
        if let Some(param) = it.param.as_ref() {
            self.visit_catch_parameter(param);
        }
        self.key_stack.push(Some("body"));
        self.visit_block_statement(&it.body);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_for_statement(&mut self, it: &ForStatement<'a>) {
        let kind = AstKind::ForStatement(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
        self.visit_span(&it.span);
        if let Some(init) = it.init.as_ref() {
            self.key_stack.push(Some("init"));
            self.visit_for_statement_init(init);
            self.key_stack.pop();
        }
        if let Some(test) = it.test.as_ref() {
            self.key_stack.push(Some("test"));
            self.visit_expression(test);
            self.key_stack.pop();
        }
        if let Some(update) = it.update.as_ref() {
            self.key_stack.push(Some("update"));
            self.visit_expression(update);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("body"));
        self.visit_statement(&it.body);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_for_in_statement(&mut self, it: &ForInStatement<'a>) {
        let kind = AstKind::ForInStatement(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
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
        self.pop_path();
    }

    fn visit_for_of_statement(&mut self, it: &ForOfStatement<'a>) {
        let kind = AstKind::ForOfStatement(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
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
        self.pop_path();
    }

    fn visit_while_statement(&mut self, it: &WhileStatement<'a>) {
        let kind = AstKind::WhileStatement(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("test"));
        self.visit_expression(&it.test);
        self.key_stack.pop();
        self.key_stack.push(Some("body"));
        self.visit_statement(&it.body);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_do_while_statement(&mut self, it: &DoWhileStatement<'a>) {
        let kind = AstKind::DoWhileStatement(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("body"));
        self.visit_statement(&it.body);
        self.key_stack.pop();
        self.key_stack.push(Some("test"));
        self.visit_expression(&it.test);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_if_statement(&mut self, it: &IfStatement<'a>) {
        let kind = AstKind::IfStatement(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("test"));
        self.visit_expression(&it.test);
        self.key_stack.pop();
        self.key_stack.push(Some("consequent"));
        self.visit_statement(&it.consequent);
        self.key_stack.pop();
        if let Some(alt) = it.alternate.as_ref() {
            self.key_stack.push(Some("alternate"));
            self.visit_statement(alt);
            self.key_stack.pop();
        }
        self.pop_path();
    }

    fn visit_try_statement(&mut self, it: &TryStatement<'a>) {
        let kind = AstKind::TryStatement(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("block"));
        self.visit_block_statement(&it.block);
        self.key_stack.pop();
        if let Some(handler) = it.handler.as_deref() {
            self.key_stack.push(Some("handler"));
            self.visit_catch_clause(handler);
            self.key_stack.pop();
        }
        if let Some(finalizer) = it.finalizer.as_deref() {
            self.key_stack.push(Some("finalizer"));
            self.visit_block_statement(finalizer);
            self.key_stack.pop();
        }
        self.pop_path();
    }

    fn visit_switch_statement(&mut self, it: &SwitchStatement<'a>) {
        let kind = AstKind::SwitchStatement(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("discriminant"));
        self.visit_expression(&it.discriminant);
        self.key_stack.pop();
        self.key_stack.push(Some("cases"));
        for case in &it.cases {
            self.visit_switch_case(case);
        }
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_switch_case(&mut self, it: &SwitchCase<'a>) {
        let kind = AstKind::SwitchCase(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
        self.visit_span(&it.span);
        if let Some(test) = it.test.as_ref() {
            self.key_stack.push(Some("test"));
            self.visit_expression(test);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("consequent"));
        self.visit_statements(&it.consequent);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_variable_declarator(&mut self, it: &VariableDeclarator<'a>) {
        let kind = AstKind::VariableDeclarator(self.alloc(it));
        self.push_path(kind, None);
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
        self.pop_path();
    }

    fn visit_assignment_expression(&mut self, it: &AssignmentExpression<'a>) {
        let kind = AstKind::AssignmentExpression(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("left"));
        self.visit_assignment_target(&it.left);
        self.key_stack.pop();
        self.key_stack.push(Some("right"));
        self.visit_expression(&it.right);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_update_expression(&mut self, it: &UpdateExpression<'a>) {
        let kind = AstKind::UpdateExpression(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("argument"));
        self.visit_simple_assignment_target(&it.argument);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_call_expression(&mut self, it: &CallExpression<'a>) {
        let kind = AstKind::CallExpression(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        if let Some(type_parameters) = it.type_arguments.as_deref() {
            self.visit_ts_type_parameter_instantiation(type_parameters);
        }
        self.key_stack.push(Some("callee"));
        self.visit_expression(&it.callee);
        self.key_stack.pop();
        self.key_stack.push(Some("arguments"));
        for (i, arg) in it.arguments.iter().enumerate() {
            self.arg_index_stack.push(Some(i));
            self.visit_argument(arg);
            self.arg_index_stack.pop();
        }
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_new_expression(&mut self, it: &NewExpression<'a>) {
        let kind = AstKind::NewExpression(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("callee"));
        self.visit_expression(&it.callee);
        self.key_stack.pop();
        if let Some(type_parameters) = it.type_arguments.as_deref() {
            self.visit_ts_type_parameter_instantiation(type_parameters);
        }
        self.key_stack.push(Some("arguments"));
        for (i, arg) in it.arguments.iter().enumerate() {
            self.arg_index_stack.push(Some(i));
            self.visit_argument(arg);
            self.arg_index_stack.pop();
        }
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_static_member_expression(&mut self, it: &StaticMemberExpression<'a>) {
        let kind = AstKind::StaticMemberExpression(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("object"));
        self.visit_expression(&it.object);
        self.key_stack.pop();
        self.key_stack.push(Some("property"));
        self.visit_identifier_name(&it.property);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_computed_member_expression(&mut self, it: &ComputedMemberExpression<'a>) {
        let kind = AstKind::ComputedMemberExpression(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("object"));
        self.visit_expression(&it.object);
        self.key_stack.pop();
        self.key_stack.push(Some("property"));
        self.visit_expression(&it.expression);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_private_field_expression(&mut self, it: &PrivateFieldExpression<'a>) {
        let kind = AstKind::PrivateFieldExpression(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("object"));
        self.visit_expression(&it.object);
        self.key_stack.pop();
        self.key_stack.push(Some("field"));
        self.visit_private_identifier(&it.field);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_expression_statement(&mut self, it: &ExpressionStatement<'a>) {
        let kind = AstKind::ExpressionStatement(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("expression"));
        self.visit_expression(&it.expression);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_return_statement(&mut self, it: &ReturnStatement<'a>) {
        let kind = AstKind::ReturnStatement(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        if let Some(arg) = it.argument.as_ref() {
            self.key_stack.push(Some("argument"));
            self.visit_expression(arg);
            self.key_stack.pop();
        }
        self.pop_path();
    }

    fn visit_throw_statement(&mut self, it: &ThrowStatement<'a>) {
        let kind = AstKind::ThrowStatement(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("argument"));
        self.visit_expression(&it.argument);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_labeled_statement(&mut self, it: &LabeledStatement<'a>) {
        // Parity fix: oxc's auto-generated `walk_labeled_statement`
        // does not push the per-child key onto `key_stack`, so a
        // BlockStatement nested directly under a LabeledStatement
        // body inherits whatever key was in scope on entry --
        // typically `"consequent"` from an outer IfStatement, even
        // though the ESTree slot label is `"body"`. The IR's
        // `scope.blockContext.key` must carry the ESTree slot label,
        // so override the visit explicitly.
        let kind = AstKind::LabeledStatement(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.visit_label_identifier(&it.label);
        self.key_stack.push(Some("body"));
        self.visit_statement(&it.body);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_sequence_expression(&mut self, it: &SequenceExpression<'a>) {
        // Parity fix: same shape as `visit_labeled_statement` -- the
        // auto-generated walker leaves the surrounding key in place
        // (frequently `"argument"` from an enclosing
        // ReturnStatement / ThrowStatement / UpdateExpression),
        // while the ESTree slot label is `"expressions"`.
        let kind = AstKind::SequenceExpression(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("expressions"));
        for expr in &it.expressions {
            self.visit_expression(expr);
        }
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_identifier_reference(&mut self, it: &IdentifierReference<'a>) {
        self.fire_reference(it.span);
        let kind = AstKind::IdentifierReference(self.alloc(it));
        self.push_path(kind, None);
        oxc_ast_visit::walk::walk_identifier_reference(self, it);
        self.pop_path();
    }

    fn visit_binding_identifier(&mut self, it: &BindingIdentifier<'a>) {
        // BindingIdentifier nodes can carry references too: a
        // `VariableDeclarator.id` with an `init` produces an init
        // write reference (`let x = 1` -> ref at offset 4). The
        // boundary creates that `ReferenceData` row at scope-build
        // time; here we fill its side-table annotations using the
        // same dispatch path as `IdentifierReference`.
        self.fire_reference(it.span);
        let kind = AstKind::BindingIdentifier(self.alloc(it));
        self.push_path(kind, None);
        oxc_ast_visit::walk::walk_binding_identifier(self, it);
        self.pop_path();
    }

    fn visit_jsx_identifier(&mut self, it: &JSXIdentifier<'a>) {
        self.fire_reference(it.span);
        let kind = AstKind::JSXIdentifier(self.alloc(it));
        self.push_path(kind, None);
        oxc_ast_visit::walk::walk_jsx_identifier(self, it);
        self.pop_path();
    }

    fn visit_identifier_name(&mut self, it: &IdentifierName<'a>) {
        // The boundary fires reference rows for `IdentifierName` only
        // when the parent is one of the ESTree-referential containers
        // (`MetaProperty`, `ImportAttribute`). `fire_reference` is a
        // no-op when no arena row carries this span, so unconditional
        // dispatch is safe — only the containers below produce a
        // matching `span_to_ref` entry.
        self.fire_reference(it.span);
        oxc_ast_visit::walk::walk_identifier_name(self, it);
    }

    fn visit_meta_property(&mut self, it: &MetaProperty<'a>) {
        let kind = AstKind::MetaProperty(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("meta"));
        self.visit_identifier_name(&it.meta);
        self.key_stack.pop();
        self.key_stack.push(Some("property"));
        self.visit_identifier_name(&it.property);
        self.key_stack.pop();
        self.pop_path();
    }

    fn visit_import_attribute(&mut self, it: &ImportAttribute<'a>) {
        let kind = AstKind::ImportAttribute(self.alloc(it));
        self.push_path(kind, None);
        self.visit_span(&it.span);
        self.key_stack.push(Some("key"));
        self.visit_import_attribute_key(&it.key);
        self.key_stack.pop();
        self.key_stack.push(Some("value"));
        self.visit_string_literal(&it.value);
        self.key_stack.pop();
        self.pop_path();
    }

    /// All non-overridden nodes still need to participate in the
    /// `path` stack so per-entry keys flow correctly through deeper
    /// callbacks; `oxc_ast_visit`'s default `enter_node` /
    /// `leave_node` give us that without per-type plumbing.
    fn enter_node(&mut self, kind: AstKind<'a>) {
        if is_explicitly_handled(&kind) {
            return;
        }
        self.push_path(kind, None);
    }

    fn leave_node(&mut self, kind: AstKind<'a>) {
        if is_explicitly_handled(&kind) {
            return;
        }
        self.pop_path();
    }
}

#[cfg(test)]
#[path = "build_analysis_visitor_test.rs"]
mod build_analysis_visitor_test;
