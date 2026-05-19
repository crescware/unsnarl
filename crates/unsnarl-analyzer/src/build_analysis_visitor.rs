//! Walker that fills the per-entity side-table annotations after the
//! eslint-scope-compatible scope build.
//!
//! Mirrors `buildAnalysisVisitor` in
//! `ts/src/pipeline/analyze/build-analysis-visitor.ts`. The TS port plugs
//! into the scope-build walk via `AnalysisVisitor` callbacks; the Rust
//! port runs a separate `oxc_ast_visit::Visit` pass after `analyze`
//! returns because several analyzer functions need full `AstKind`
//! handles (`expression_statement_container`, `find_reference_owners`,
//! `case_falls_through`, `case_exits_function`, `format_case_test`) or
//! per-entry keys (`find_predicate_container`, `if_chain_root_offset`)
//! — both of which the boundary's materialised-path callback shape
//! drops.
//!
//! The walker is keyed against the arena built by the boundary: scope
//! rows are looked up by `scope.block.span`, reference rows by
//! `reference.identifier.span`. Encountered AST nodes whose spans do
//! not match any arena row are simply skipped (e.g. a `BlockStatement`
//! that is the body of a `CatchClause` — the boundary's
//! `skip_block_scope` table makes those reuse the catch scope rather
//! than allocating a new one).

use std::collections::HashMap;

use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentExpression, BindingIdentifier, BlockStatement,
    CallExpression, CatchClause, Class, ComputedMemberExpression, DoWhileStatement,
    ExpressionStatement, ForInStatement, ForOfStatement, ForStatement, Function,
    IdentifierReference, IfStatement, JSXIdentifier, NewExpression, PrivateFieldExpression,
    Program, ReturnStatement, StaticMemberExpression, SwitchCase, SwitchStatement, ThrowStatement,
    TryStatement, UpdateExpression, VariableDeclarator, WhileStatement,
};
use oxc_ast::AstKind;
use oxc_ast_visit::Visit;
use oxc_span::Span;
use oxc_syntax::scope::ScopeFlags;

use unsnarl_annotations::{ReferenceAnnotation, ScopeAnnotation};
use unsnarl_boundary_eslint_scope::materialise::ast_node_of;
use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};
use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope::block_context::CaseClauseBlockContext;
use unsnarl_ir::scope::BlockContext;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::{IrArena, ReferenceId, ScopeId, SourceOffset};
use unsnarl_oxc_parity::AstType;

use crate::annotations_impl::AnnotationsImpl;
use crate::block_context_of::block_context_of;
use crate::case_exits_function::case_exits_function;
use crate::case_falls_through::case_falls_through;
use crate::expression_statement_container::build_expression_statement_container;
use crate::find_completion::find_completion;
use crate::find_jsx_element_span::find_jsx_element_span;
use crate::find_predicate_container::find_predicate_container;
use crate::format_case_test::format_case_test;
use crate::owner::{
    all_binding_variables, assignment_target_variables, locate_reference_owner_slot, OwnerLookup,
};
use crate::path_entry::{ArrowBodyInfo, PathEntry};
use crate::reference_call_receiver::reference_call_receiver_flags;

const ZERO_DEPTHS: NestingDepths = NestingDepths {
    function: NestingDepth(0),
    r#if: NestingDepth(0),
    r#for: NestingDepth(0),
    r#while: NestingDepth(0),
    switch: NestingDepth(0),
    try_catch_finally: NestingDepth(0),
    block: NestingDepth(0),
};

/// Internal walk-time frame: the live `AstKind` handle plus the key it
/// occupies on its parent (the slot's field name) plus, for
/// `ArrowFunctionExpression`, the body-shape side-channel
/// `find_completion` needs to distinguish expression-body arrows from
/// block-body arrows.
struct PathFrame<'a> {
    kind: AstKind<'a>,
    key: Option<&'static str>,
    arrow_body: Option<ArrowBodyInfo>,
}

pub(crate) struct BuildAnalysisVisitor<'a, 'arena> {
    raw: &'arena str,
    arena: &'arena IrArena,
    annotations: &'arena mut AnnotationsImpl,
    nesting_depths: &'arena HashMap<u32, NestingDepths>,
    span_to_scope: &'arena HashMap<(u32, u32), ScopeId>,
    span_to_ref: &'arena HashMap<(u32, u32), ReferenceId>,
    key_stack: Vec<Option<&'static str>>,
    path: Vec<PathFrame<'a>>,
}

impl<'a, 'arena> BuildAnalysisVisitor<'a, 'arena> {
    pub(crate) fn new(
        raw: &'arena str,
        arena: &'arena IrArena,
        annotations: &'arena mut AnnotationsImpl,
        nesting_depths: &'arena HashMap<u32, NestingDepths>,
        span_to_scope: &'arena HashMap<(u32, u32), ScopeId>,
        span_to_ref: &'arena HashMap<(u32, u32), ReferenceId>,
    ) -> Self {
        Self {
            raw,
            arena,
            annotations,
            nesting_depths,
            span_to_scope,
            span_to_ref,
            key_stack: Vec::new(),
            path: Vec::new(),
        }
    }

    fn current_key(&self) -> Option<&'static str> {
        self.key_stack.last().copied().flatten()
    }

    fn materialise_path(&self) -> Vec<PathEntry> {
        self.path
            .iter()
            .map(|f| PathEntry {
                node: ast_node_of(&f.kind),
                key: f.key,
                arrow_body: f.arrow_body,
            })
            .collect()
    }

    fn parent_ast_node(&self) -> Option<AstNode> {
        self.path.last().map(|f| ast_node_of(&f.kind))
    }

    /// Fill the `ScopeAnnotation` row for a scope whose block matches
    /// `span`. Returns silently when `span` does not map to any scope
    /// (the BlockStatement under a CatchClause case).
    fn fire_scope(&mut self, span: Span, kind: &AstKind<'a>) {
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
        let path_entries = self.materialise_path();
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
                .map(|expr| format_case_test(expr, self.raw));
            let block_context = parent_node.as_ref().zip(key).map(|(parent, k)| {
                BlockContext::CaseClause(CaseClauseBlockContext::new(
                    parent.r#type.clone(),
                    k.to_string(),
                    SourceOffset(parent.span.start),
                    case_test,
                ))
            });
            let falls_through = case_falls_through(&switch_case.consequent);
            let exits_function = case_exits_function(&switch_case.consequent);
            (block_context, falls_through, exits_function)
        } else if matches!(scope_type, ScopeType::Function) {
            (None, false, false)
        } else {
            let block_context = block_context_of(parent_node.as_ref(), key, &path_entries);
            (block_context, false, false)
        };
        self.annotations.set_scope(
            scope_id,
            ScopeAnnotation {
                block_context,
                falls_through,
                exits_function,
                nesting_depths,
            },
        );
    }

    /// Fill the `ReferenceAnnotation` row for an identifier-shaped
    /// node (`IdentifierReference`, `BindingIdentifier`, or
    /// `JSXIdentifier`) whose span matches an entry in `span_to_ref`.
    fn fire_reference(&mut self, span: Span) {
        let Some(&ref_id) = self.span_to_ref.get(&(span.start, span.end)) else {
            return;
        };
        let scope = self.arena.references[ref_id].from;
        let parent_node = self.parent_ast_node();
        let parent_type = parent_node.as_ref().map(|n| n.r#type.clone());
        let parent_offset = parent_node.as_ref().map(|n| n.span.start);
        let key = self.current_key();
        let path_entries = self.materialise_path();
        let owners = match locate_reference_owner_slot(&path_entries) {
            OwnerLookup::None | OwnerLookup::Boundary => Vec::new(),
            OwnerLookup::VariableDeclarator { path_index } => {
                let kind = &self.path[path_index].kind;
                match kind {
                    AstKind::VariableDeclarator(vd) => {
                        all_binding_variables(&vd.id, scope, self.arena)
                    }
                    _ => Vec::new(),
                }
            }
            OwnerLookup::AssignmentExpression { path_index } => {
                let kind = &self.path[path_index].kind;
                match kind {
                    AstKind::AssignmentExpression(ae) => {
                        assignment_target_variables(&ae.left, scope, self.arena)
                    }
                    _ => Vec::new(),
                }
            }
        };
        let flags = reference_call_receiver_flags(parent_type.as_ref(), key);
        let predicate_container =
            find_predicate_container(parent_type.as_ref(), parent_offset, key, &path_entries);
        let completion = find_completion(&path_entries);
        let jsx_element = find_jsx_element_span(&path_entries);
        let expression_statement_container = self.path.iter().rev().find_map(|f| match &f.kind {
            AstKind::ExpressionStatement(es) => Some(build_expression_statement_container(
                es.span,
                Some(&es.expression),
            )),
            _ => None,
        });
        self.annotations.set_reference(
            ref_id,
            ReferenceAnnotation {
                owners,
                flags,
                predicate_container,
                completion,
                jsx_element,
                expression_statement_container,
            },
        );
    }

    fn push_path(&mut self, kind: AstKind<'a>, arrow_body: Option<ArrowBodyInfo>) {
        let key = self.current_key();
        self.path.push(PathFrame {
            kind,
            key,
            arrow_body,
        });
    }

    fn pop_path(&mut self) {
        self.path.pop();
    }
}

impl<'a, 'arena> Visit<'a> for BuildAnalysisVisitor<'a, 'arena> {
    fn visit_program(&mut self, it: &Program<'a>) {
        let kind = AstKind::Program(self.alloc(it));
        // Match the normalised Program span the boundary stamps on
        // the global scope's `block` -- start at the first
        // hashbang / directive / body node offset (the TS
        // `oxc-parser` shape), end at `program.span.end`. Using
        // `it.span` here would key into `span_to_scope` with the
        // oxc-Rust native span and miss the global scope when the
        // source has leading comments.
        let start = it
            .hashbang
            .as_ref()
            .map(|h| h.span.start)
            .or_else(|| it.directives.first().map(|d| d.span.start))
            .or_else(|| it.body.first().map(|s| oxc_span::GetSpan::span(s).start))
            .unwrap_or(it.span.start);
        let program_span = Span::new(start, it.span.end);
        self.fire_scope(program_span, &kind);
        self.push_path(kind, None);
        self.visit_span(&it.span);
        if let Some(hashbang) = it.hashbang.as_ref() {
            self.visit_hashbang(hashbang);
        }
        self.visit_directives(&it.directives);
        self.visit_statements(&it.body);
        self.pop_path();
    }

    fn visit_block_statement(&mut self, it: &BlockStatement<'a>) {
        let kind = AstKind::BlockStatement(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
        oxc_ast_visit::walk::walk_block_statement(self, it);
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
        self.visit_function_body(&it.body);
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
            self.visit_catch_clause(handler);
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
        for case in &it.cases {
            self.visit_switch_case(case);
        }
        self.pop_path();
    }

    fn visit_switch_case(&mut self, it: &SwitchCase<'a>) {
        let kind = AstKind::SwitchCase(self.alloc(it));
        self.fire_scope(it.span, &kind);
        self.push_path(kind, None);
        oxc_ast_visit::walk::walk_switch_case(self, it);
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
        self.visit_arguments(&it.arguments);
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
        self.visit_arguments(&it.arguments);
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

fn is_explicitly_handled(kind: &AstKind<'_>) -> bool {
    matches!(
        kind,
        AstKind::Program(_)
            | AstKind::BlockStatement(_)
            | AstKind::Function(_)
            | AstKind::ArrowFunctionExpression(_)
            | AstKind::Class(_)
            | AstKind::CatchClause(_)
            | AstKind::ForStatement(_)
            | AstKind::ForInStatement(_)
            | AstKind::ForOfStatement(_)
            | AstKind::WhileStatement(_)
            | AstKind::DoWhileStatement(_)
            | AstKind::IfStatement(_)
            | AstKind::TryStatement(_)
            | AstKind::SwitchStatement(_)
            | AstKind::SwitchCase(_)
            | AstKind::VariableDeclarator(_)
            | AstKind::AssignmentExpression(_)
            | AstKind::UpdateExpression(_)
            | AstKind::CallExpression(_)
            | AstKind::NewExpression(_)
            | AstKind::StaticMemberExpression(_)
            | AstKind::ComputedMemberExpression(_)
            | AstKind::PrivateFieldExpression(_)
            | AstKind::ExpressionStatement(_)
            | AstKind::ReturnStatement(_)
            | AstKind::ThrowStatement(_)
            | AstKind::IdentifierReference(_)
            | AstKind::BindingIdentifier(_)
            | AstKind::JSXIdentifier(_)
    )
}
