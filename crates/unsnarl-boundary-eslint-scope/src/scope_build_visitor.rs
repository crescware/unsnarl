//! Walker that drives the eslint-scope-compatible scope build.
//!
//! Implements `oxc_ast_visit::Visit<'a>` directly: each per-AST-shape
//! `visit_*` override plays the role of an enter / leave dispatcher
//! arm, recording the parent-slot key on `key_stack` around each
//! child visit so the downstream `classify/*` layer sees the correct
//! `(parent, key)` ancestor pair.
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
    AccessorProperty, ArrowFunctionExpression, AssignmentExpression, BindingIdentifier,
    BlockStatement, CallExpression, CatchClause, Class, ExportAllDeclaration,
    ExportNamedDeclaration, ExportSpecifier, ForInStatement, ForOfStatement, ForStatement,
    FormalParameter, FormalParameterRest, Function, IdentifierName, IdentifierReference,
    ImportAttribute, ImportDefaultSpecifier, ImportNamespaceSpecifier, ImportSpecifier,
    JSXAttribute, JSXIdentifier, JSXMemberExpression, JSXOpeningElement, MetaProperty,
    NewExpression, PropertyDefinition, SwitchCase, SwitchStatement, TSAsExpression,
    TSInstantiationExpression, TSSatisfiesExpression, TSTypeAssertion, TaggedTemplateExpression,
    UpdateExpression, VariableDeclarator,
};
use oxc_ast::AstKind;
use oxc_syntax::scope::ScopeFlags;

use unsnarl_ir::ids::ScopeId;
use unsnarl_oxc_parity::{is_type_only_subtree, AstType};

use crate::enter_block::enter_block;
use crate::enter_catch::enter_catch;
use crate::enter_class::enter_class;
use crate::enter_for::{enter_for_in_statement, enter_for_of_statement, enter_for_statement};
use crate::enter_function::{enter_arrow_function_expression, enter_function};
use crate::enter_switch::enter_switch;
use crate::enter_switch_case::enter_switch_case;
use crate::handle_identifier_reference::handle_identifier_reference;
use crate::materialise::{ast_node_of, ast_type_of, materialise_path};
use crate::skip_block_scope::skip_block_scope;
use crate::state::{pop_scope, ScopeBuilderState};
use crate::visitor::AnalysisVisitor;
use crate::walk::PathEntry;

pub(crate) struct ScopeBuildVisitor<'a, 'v> {
    pub(crate) state: &'v mut ScopeBuilderState,
    pub(crate) visitor: &'v mut dyn AnalysisVisitor,
    pub(crate) raw: &'v str,
    pub(crate) key_stack: Vec<Option<&'static str>>,
    pub(crate) path: Vec<PathEntry<'a>>,
    /// Count of currently-active TypeScript type-only subtrees
    /// containing the cursor. Type-only nodes (`TSInterfaceDeclaration`,
    /// `typeAnnotation` slot, ...) must not produce reference rows;
    /// `oxc_ast_visit` has no "skip" return, so we walk the subtree
    /// normally but short-circuit `handle_identifier_reference` while
    /// this counter is positive. See
    /// `unsnarl_oxc_parity::is_type_only_subtree` for the membership.
    pub(crate) type_only_depth: u32,
}

impl<'a, 'v> ScopeBuildVisitor<'a, 'v> {
    pub(crate) fn new(
        state: &'v mut ScopeBuilderState,
        visitor: &'v mut dyn AnalysisVisitor,
        raw: &'v str,
    ) -> Self {
        Self {
            state,
            visitor,
            raw,
            key_stack: Vec::new(),
            path: Vec::new(),
            type_only_depth: 0,
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

    fn fire_on_scope(&mut self, scope_id: ScopeId) {
        let parent = self.parent_kind();
        let parent_node = parent.as_ref().map(ast_node_of);
        let key = self.current_key();
        let path_materialised = materialise_path(&self.path);
        self.visitor.on_scope(
            scope_id,
            parent_node.as_ref(),
            key,
            &path_materialised,
            self.state,
        );
    }
}

impl<'a, 'v> oxc_ast_visit::Visit<'a> for ScopeBuildVisitor<'a, 'v> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        let key = self.current_key();
        let ty = ast_type_for_skip(&kind);
        if is_type_only_subtree(&ty, key) {
            self.type_only_depth += 1;
        }
        self.path.push(PathEntry { node: kind, key });
    }

    fn leave_node(&mut self, kind: AstKind<'a>) {
        let entry_key = self.path.last().and_then(|e| e.key);
        let ty = ast_type_for_skip(&kind);
        if is_type_only_subtree(&ty, entry_key) {
            self.type_only_depth = self.type_only_depth.saturating_sub(1);
        }
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
        let scope_id = enter_block(self.state, it, self.raw);
        self.fire_on_scope(scope_id);
        oxc_ast_visit::walk::walk_block_statement(self, it);
        pop_scope(self.state);
    }

    fn visit_function(&mut self, it: &Function<'a>, flags: ScopeFlags) {
        // `declare function f(): void;` is a type-only declaration
        // (oxc tags it as `FunctionType::TSDeclareFunction`); no
        // function scope / variable should be created for it. Still
        // push the path entry so `type_only_depth` is maintained for
        // any descendants, but don't enter a function scope.
        if matches!(it.r#type, oxc_ast::ast::FunctionType::TSDeclareFunction) {
            let kind = AstKind::Function(self.alloc(it));
            self.enter_node(kind);
            self.leave_node(kind);
            return;
        }
        // Inside a TypeScript type-only subtree (`abstract m(): void;`
        // is a FunctionExpression child of `TSAbstractMethodDefinition`),
        // no function scope should be created. oxc's `Visit` trait has
        // no "skip" return, so we instead enter the node (for
        // `type_only_depth` bookkeeping) and walk the body without
        // creating a function scope.
        if self.type_only_depth > 0 {
            let kind = AstKind::Function(self.alloc(it));
            self.enter_node(kind);
            self.leave_node(kind);
            return;
        }
        let scope_id = enter_function(self.state, it, self.raw);
        self.fire_on_scope(scope_id);
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
            self.key_stack.push(Some("typeParameters"));
            self.visit_ts_type_parameter_declaration(type_parameters);
            self.key_stack.pop();
        }
        if let Some(this_param) = it.this_param.as_deref() {
            self.visit_ts_this_parameter(this_param);
        }
        self.key_stack.push(Some("params"));
        self.visit_formal_parameters(&it.params);
        self.key_stack.pop();
        if let Some(return_type) = it.return_type.as_deref() {
            self.key_stack.push(Some("returnType"));
            self.visit_ts_type_annotation(return_type);
            self.key_stack.pop();
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
        let scope_id = enter_arrow_function_expression(self.state, it, self.raw);
        self.fire_on_scope(scope_id);
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
            self.key_stack.push(Some("typeParameters"));
            self.visit_ts_type_parameter_declaration(type_parameters);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("params"));
        self.visit_formal_parameters(&it.params);
        self.key_stack.pop();
        if let Some(return_type) = it.return_type.as_deref() {
            self.key_stack.push(Some("returnType"));
            self.visit_ts_type_annotation(return_type);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("body"));
        self.visit_function_body(&it.body);
        self.key_stack.pop();
        self.leave_scope();
        self.leave_node(kind);
        pop_scope(self.state);
    }

    fn visit_formal_parameter(&mut self, it: &FormalParameter<'a>) {
        let kind = AstKind::FormalParameter(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.visit_decorators(&it.decorators);
        // TypeScript parameter property (`constructor(public x: number)`
        // etc.): the inner identifier must fall through to
        // `classify_ordinary_reference` -- i.e. become a plain read
        // reference (resolving as an implicit global), not a binding.
        // oxc records it on the FormalParameter directly via
        // `accessibility` / `readonly` / `override`, so detect those
        // flags here and skip the `pattern` slot key so
        // `is_direct_binding`'s `FormalParameter.pattern` rule does
        // not fire.
        let is_param_property = it.accessibility.is_some() || it.readonly || it.r#override;
        if is_param_property {
            self.visit_binding_pattern(&it.pattern);
        } else {
            self.key_stack.push(Some("pattern"));
            self.visit_binding_pattern(&it.pattern);
            self.key_stack.pop();
        }
        if let Some(type_annotation) = it.type_annotation.as_deref() {
            self.key_stack.push(Some("typeAnnotation"));
            self.visit_ts_type_annotation(type_annotation);
            self.key_stack.pop();
        }
        if let Some(initializer) = it.initializer.as_deref() {
            // oxc routes `function f(a = b)` as
            // `FormalParameter { pattern: a, initializer: b }`, but
            // `classify` / `find_binding_root_context` expect the
            // ESTree `AssignmentPattern { left: a, right: b }` shape:
            // `b` is the direct child of `AssignmentPattern` and
            // `find_binding_root_context` walks `AssignmentPattern`
            // (a pattern step) up to `Function@params` and returns
            // `param`. Push an `"initializer"` key so the slot is
            // recognizable. Identifiers nested inside an expression
            // (e.g. `a + b` in `c = a + b`) keep their immediate
            // expression parent and still classify as references.
            self.key_stack.push(Some("initializer"));
            self.visit_expression(initializer);
            self.key_stack.pop();
        }
        self.leave_node(kind);
    }

    fn visit_formal_parameter_rest(&mut self, it: &FormalParameterRest<'a>) {
        // `function f(...rest: T[])` -- the rest parameter's
        // `type_annotation` is a TypeScript-only subtree. oxc's
        // auto-generated walker descends into it without recording
        // the parent-slot key, so identifiers inside (a named type
        // like `VisualNode`) leak through `is_type_only_subtree` and
        // emit extra `Reference` rows. Push `"typeAnnotation"` around
        // the slot so it is recognized as type-only.
        let kind = AstKind::FormalParameterRest(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("decorators"));
        self.visit_decorators(&it.decorators);
        self.key_stack.pop();
        self.visit_binding_rest_element(&it.rest);
        if let Some(type_annotation) = it.type_annotation.as_deref() {
            self.key_stack.push(Some("typeAnnotation"));
            self.visit_ts_type_annotation(type_annotation);
            self.key_stack.pop();
        }
        self.leave_node(kind);
    }

    fn visit_class(&mut self, it: &Class<'a>) {
        let scope_id = enter_class(self.state, it);
        self.fire_on_scope(scope_id);
        let kind = AstKind::Class(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.visit_decorators(&it.decorators);
        if let Some(id) = it.id.as_ref() {
            self.key_stack.push(Some("id"));
            self.visit_binding_identifier(id);
            self.key_stack.pop();
        }
        if let Some(type_parameters) = it.type_parameters.as_deref() {
            self.key_stack.push(Some("typeParameters"));
            self.visit_ts_type_parameter_declaration(type_parameters);
            self.key_stack.pop();
        }
        if let Some(super_class) = it.super_class.as_ref() {
            self.key_stack.push(Some("superClass"));
            self.visit_expression(super_class);
            self.key_stack.pop();
        }
        if let Some(super_type_arguments) = it.super_type_arguments.as_deref() {
            self.key_stack.push(Some("superTypeArguments"));
            self.visit_ts_type_parameter_instantiation(super_type_arguments);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("implements"));
        for ts_class_implements in &it.implements {
            self.visit_ts_class_implements(ts_class_implements);
        }
        self.key_stack.pop();
        self.key_stack.push(Some("body"));
        self.visit_class_body(&it.body);
        self.key_stack.pop();
        self.leave_node(kind);
        pop_scope(self.state);
    }

    fn visit_property_definition(&mut self, it: &PropertyDefinition<'a>) {
        // `class { items: Diagnostic[] = [] }` -- the property's
        // `type_annotation` is a TypeScript-only subtree. oxc's
        // auto-generated walker descends into it without recording
        // the parent-slot key, so identifiers inside (a named type
        // like `Diagnostic`) leak through `is_type_only_subtree` and
        // emit extra `Reference` rows. Push `"typeAnnotation"` here
        // so the slot is recognized as type-only. Decorators / key /
        // value remain runtime slots and keep their own keys.
        let kind = AstKind::PropertyDefinition(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("decorators"));
        self.visit_decorators(&it.decorators);
        self.key_stack.pop();
        self.key_stack.push(Some("key"));
        self.visit_property_key(&it.key);
        self.key_stack.pop();
        if let Some(type_annotation) = it.type_annotation.as_deref() {
            self.key_stack.push(Some("typeAnnotation"));
            self.visit_ts_type_annotation(type_annotation);
            self.key_stack.pop();
        }
        if let Some(value) = &it.value {
            self.key_stack.push(Some("value"));
            self.visit_expression(value);
            self.key_stack.pop();
        }
        self.leave_node(kind);
    }

    fn visit_accessor_property(&mut self, it: &AccessorProperty<'a>) {
        // `class { accessor name: T }` -- same `typeAnnotation`-key
        // omission as `PropertyDefinition` (see
        // `visit_property_definition`).
        let kind = AstKind::AccessorProperty(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("decorators"));
        self.visit_decorators(&it.decorators);
        self.key_stack.pop();
        self.key_stack.push(Some("key"));
        self.visit_property_key(&it.key);
        self.key_stack.pop();
        if let Some(type_annotation) = it.type_annotation.as_deref() {
            self.key_stack.push(Some("typeAnnotation"));
            self.visit_ts_type_annotation(type_annotation);
            self.key_stack.pop();
        }
        if let Some(value) = &it.value {
            self.key_stack.push(Some("value"));
            self.visit_expression(value);
            self.key_stack.pop();
        }
        self.leave_node(kind);
    }

    fn visit_catch_clause(&mut self, it: &CatchClause<'a>) {
        // `classify/*` recognises `(parent=CatchClause, key="param")`
        // as a direct binding, matching the ESTree shape where the
        // `CatchParameter` wrapper is collapsed and the param is a
        // bare `BindingPattern`. Walk the pattern directly (skipping
        // the `CatchParameter` wrapper) and push `"param"` so
        // `is_direct_binding` fires and no write reference is created
        // for `err` in `catch (err) { ... }`.
        let scope_id = enter_catch(self.state, it, self.raw);
        self.fire_on_scope(scope_id);
        let kind = AstKind::CatchClause(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.enter_scope(ScopeFlags::CatchClause, &it.scope_id);
        if let Some(param) = it.param.as_ref() {
            self.key_stack.push(Some("param"));
            self.visit_binding_pattern(&param.pattern);
            self.key_stack.pop();
            if let Some(type_annotation) = param.type_annotation.as_deref() {
                self.key_stack.push(Some("typeAnnotation"));
                self.visit_ts_type_annotation(type_annotation);
                self.key_stack.pop();
            }
        }
        self.visit_block_statement(&it.body);
        self.leave_scope();
        self.leave_node(kind);
        pop_scope(self.state);
    }

    fn visit_for_statement(&mut self, it: &ForStatement<'a>) {
        let scope_id = enter_for_statement(self.state, it, self.raw);
        self.fire_on_scope(scope_id);
        oxc_ast_visit::walk::walk_for_statement(self, it);
        pop_scope(self.state);
    }

    fn visit_for_in_statement(&mut self, it: &ForInStatement<'a>) {
        let scope_id = enter_for_in_statement(self.state, it, self.raw);
        self.fire_on_scope(scope_id);
        oxc_ast_visit::walk::walk_for_in_statement(self, it);
        pop_scope(self.state);
    }

    fn visit_for_of_statement(&mut self, it: &ForOfStatement<'a>) {
        let scope_id = enter_for_of_statement(self.state, it, self.raw);
        self.fire_on_scope(scope_id);
        oxc_ast_visit::walk::walk_for_of_statement(self, it);
        pop_scope(self.state);
    }

    fn visit_switch_statement(&mut self, it: &SwitchStatement<'a>) {
        let scope_id = enter_switch(self.state, it);
        self.fire_on_scope(scope_id);
        oxc_ast_visit::walk::walk_switch_statement(self, it);
        pop_scope(self.state);
    }

    fn visit_switch_case(&mut self, it: &SwitchCase<'a>) {
        let scope_id = enter_switch_case(self.state, it, self.raw);
        self.fire_on_scope(scope_id);
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
            self.key_stack.push(Some("typeAnnotation"));
            self.visit_ts_type_annotation(type_annotation);
            self.key_stack.pop();
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

    fn visit_import_specifier(&mut self, it: &ImportSpecifier<'a>) {
        // `classify/is_direct_binding` requires `key == Some("local")`
        // on `ImportSpecifier` to recognise the local-binding slot,
        // and `is_skip_context` requires `key == Some("imported")` to
        // skip the imported-name slot (which can be a JSXIdentifier
        // shape in some module-export grammars).
        let kind = AstKind::ImportSpecifier(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("imported"));
        self.visit_module_export_name(&it.imported);
        self.key_stack.pop();
        self.key_stack.push(Some("local"));
        self.visit_binding_identifier(&it.local);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_import_default_specifier(&mut self, it: &ImportDefaultSpecifier<'a>) {
        let kind = AstKind::ImportDefaultSpecifier(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("local"));
        self.visit_binding_identifier(&it.local);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_import_namespace_specifier(&mut self, it: &ImportNamespaceSpecifier<'a>) {
        let kind = AstKind::ImportNamespaceSpecifier(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("local"));
        self.visit_binding_identifier(&it.local);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_export_named_declaration(&mut self, it: &ExportNamedDeclaration<'a>) {
        // oxc's auto-generated `walk_export_named_declaration` descends
        // into each child slot without recording the parent-slot key,
        // so a `(parent=ExportNamedDeclaration, key="body")` mis-tag
        // leaks down to the contained `Declaration`/specifier
        // identifiers from whatever the surrounding statement-list
        // pushed (typically `"body"` from `Program.body`). The ESTree
        // visitorKey list for `ExportNamedDeclaration` is
        // `["declaration", "specifiers", "source", "attributes"]`;
        // push each key around the matching child so `classify_*` /
        // `is_skip_context` observe the right slot label downstream.
        let kind = AstKind::ExportNamedDeclaration(self.alloc(it));
        self.enter_node(kind);
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
        self.leave_node(kind);
    }

    fn visit_ts_as_expression(&mut self, it: &TSAsExpression<'a>) {
        // `x as T` -- oxc's auto-generated walker descends into
        // `type_annotation` without recording the parent-slot key, so
        // `is_type_only_subtree` never observes
        // `key = Some("typeAnnotation")` for the inner type subtree
        // and any `Identifier` inside (e.g. the `const` of `as const`,
        // or a named type like `as UnsnarlPlugin`) is mis-handled as a
        // runtime reference. Push `"typeAnnotation"` around the type
        // slot so it increments `type_only_depth` and downstream
        // identifier-reference creation short-circuits.
        let kind = AstKind::TSAsExpression(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("expression"));
        self.visit_expression(&it.expression);
        self.key_stack.pop();
        self.key_stack.push(Some("typeAnnotation"));
        self.visit_ts_type(&it.type_annotation);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_ts_satisfies_expression(&mut self, it: &TSSatisfiesExpression<'a>) {
        // `x satisfies T` -- same `typeAnnotation`-key omission as
        // `TSAsExpression` (see `visit_ts_as_expression`).
        let kind = AstKind::TSSatisfiesExpression(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("expression"));
        self.visit_expression(&it.expression);
        self.key_stack.pop();
        self.key_stack.push(Some("typeAnnotation"));
        self.visit_ts_type(&it.type_annotation);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_ts_type_assertion(&mut self, it: &TSTypeAssertion<'a>) {
        // `<T>x` (legacy TypeScript cast) -- same `typeAnnotation`-key
        // omission as `TSAsExpression` (see `visit_ts_as_expression`).
        let kind = AstKind::TSTypeAssertion(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("typeAnnotation"));
        self.visit_ts_type(&it.type_annotation);
        self.key_stack.pop();
        self.key_stack.push(Some("expression"));
        self.visit_expression(&it.expression);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_ts_instantiation_expression(&mut self, it: &TSInstantiationExpression<'a>) {
        // `f<T>` -- the type-argument slot is type-only; push
        // `"typeArguments"` around the `type_arguments` visit so
        // `is_type_only_subtree` recognises it. Without it, an
        // identifier inside the type-argument list would be mis-tagged
        // as a runtime reference.
        let kind = AstKind::TSInstantiationExpression(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("expression"));
        self.visit_expression(&it.expression);
        self.key_stack.pop();
        self.key_stack.push(Some("typeArguments"));
        self.visit_ts_type_parameter_instantiation(&it.type_arguments);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_call_expression(&mut self, it: &CallExpression<'a>) {
        // `f<T>(arg)` -- the type arguments slot is TypeScript-only.
        // oxc's auto-generated walker descends into `type_arguments`
        // without recording the parent-slot key, so an identifier
        // inside (a named type like `f<UserModel>(...)`) would slip
        // through `is_type_only_subtree` and be classified as a
        // runtime reference. Push `"typeArguments"` around the slot.
        let kind = AstKind::CallExpression(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("callee"));
        self.visit_expression(&it.callee);
        self.key_stack.pop();
        if let Some(type_arguments) = it.type_arguments.as_deref() {
            self.key_stack.push(Some("typeArguments"));
            self.visit_ts_type_parameter_instantiation(type_arguments);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("arguments"));
        self.visit_arguments(&it.arguments);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_new_expression(&mut self, it: &NewExpression<'a>) {
        // `new Foo<T>(arg)` -- same `typeArguments`-key omission as
        // `CallExpression` (see `visit_call_expression`).
        let kind = AstKind::NewExpression(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("callee"));
        self.visit_expression(&it.callee);
        self.key_stack.pop();
        if let Some(type_arguments) = it.type_arguments.as_deref() {
            self.key_stack.push(Some("typeArguments"));
            self.visit_ts_type_parameter_instantiation(type_arguments);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("arguments"));
        self.visit_arguments(&it.arguments);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_tagged_template_expression(&mut self, it: &TaggedTemplateExpression<'a>) {
        // `` tag<T>`literal` `` -- same `typeArguments`-key omission
        // as `CallExpression` (see `visit_call_expression`).
        let kind = AstKind::TaggedTemplateExpression(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("tag"));
        self.visit_expression(&it.tag);
        self.key_stack.pop();
        if let Some(type_arguments) = it.type_arguments.as_deref() {
            self.key_stack.push(Some("typeArguments"));
            self.visit_ts_type_parameter_instantiation(type_arguments);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("quasi"));
        self.visit_template_literal(&it.quasi);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_jsx_opening_element(&mut self, it: &JSXOpeningElement<'a>) {
        // `<Foo<T> .../>` -- same `typeArguments`-key omission as
        // `CallExpression` (see `visit_call_expression`).
        let kind = AstKind::JSXOpeningElement(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("name"));
        self.visit_jsx_element_name(&it.name);
        self.key_stack.pop();
        if let Some(type_arguments) = it.type_arguments.as_deref() {
            self.key_stack.push(Some("typeArguments"));
            self.visit_ts_type_parameter_instantiation(type_arguments);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("attributes"));
        self.visit_jsx_attribute_items(&it.attributes);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_export_all_declaration(&mut self, it: &ExportAllDeclaration<'a>) {
        // `export * as default from './base.js'` -- the `exported`
        // slot is a regular `Identifier` reference that resolves to
        // an implicit global. oxc represents the slot as
        // `Option<ModuleExportName>` and the `Identifier` shape is
        // `ModuleExportName::IdentifierName`, so the per-slot routing
        // for `IdentifierName` (in `visit_identifier_name` below)
        // needs the `(parent = ExportAllDeclaration, key =
        // "exported")` ancestor pair to be present on `key_stack`.
        // Push it explicitly here -- oxc's auto-generated walker
        // would otherwise leave whatever surrounding label was in
        // scope on entry.
        let kind = AstKind::ExportAllDeclaration(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        if let Some(exported) = &it.exported {
            self.key_stack.push(Some("exported"));
            self.visit_module_export_name(exported);
            self.key_stack.pop();
        }
        self.key_stack.push(Some("source"));
        self.visit_string_literal(&it.source);
        self.key_stack.pop();
        if let Some(with_clause) = &it.with_clause {
            self.key_stack.push(Some("attributes"));
            self.visit_with_clause(with_clause);
            self.key_stack.pop();
        }
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
        if self.type_only_depth == 0 {
            let parent = self.parent_kind();
            let key = self.current_key();
            // oxc represents JSX object / element-name identifiers as
            // plain `IdentifierReference` nodes (per the JSXElementName /
            // JSXMemberExpressionObject enums), but the IR expects
            // them to carry the `JSXIdentifier` shape on the resulting
            // reference / implicit-global Definition rows. Normalise
            // the type here.
            let ast_type = if matches!(
                parent,
                Some(AstKind::JSXMemberExpression(_)) | Some(AstKind::JSXOpeningElement(_))
            ) {
                AstType::JSXIdentifier
            } else {
                AstType::Identifier
            };
            handle_identifier_reference(
                self.state,
                self.visitor,
                parent.as_ref(),
                key,
                &self.path,
                it.name.as_str(),
                it.span,
                ast_type,
            );
        }
        oxc_ast_visit::walk::walk_identifier_reference(self, it);
    }

    fn visit_binding_identifier(&mut self, it: &BindingIdentifier<'a>) {
        // Routes `BindingIdentifier` through the shared classification
        // path: `classify_identifier` decides whether the slot is a
        // plain binding (no reference), a write reference with
        // `init = true` (`let x = 1`'s `x`), or a pattern-step
        // binding.
        if self.type_only_depth == 0 {
            let parent = self.parent_kind();
            let key = self.current_key();
            handle_identifier_reference(
                self.state,
                self.visitor,
                parent.as_ref(),
                key,
                &self.path,
                it.name.as_str(),
                it.span,
                AstType::Identifier,
            );
        }
        oxc_ast_visit::walk::walk_binding_identifier(self, it);
    }

    fn visit_jsx_attribute(&mut self, it: &JSXAttribute<'a>) {
        // `classify/is_skip_context` treats `JSXAttribute.name` as a
        // skip slot; push the key so the JSXIdentifier child sees it
        // as its parent slot.
        let kind = AstKind::JSXAttribute(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("name"));
        self.visit_jsx_attribute_name(&it.name);
        self.key_stack.pop();
        if let Some(value) = it.value.as_ref() {
            self.key_stack.push(Some("value"));
            self.visit_jsx_attribute_value(value);
            self.key_stack.pop();
        }
        self.leave_node(kind);
    }

    fn visit_jsx_member_expression(&mut self, it: &JSXMemberExpression<'a>) {
        // `classify/is_skip_context` treats `JSXMemberExpression.property`
        // as a skip slot; push the `"property"` key so the JSXIdentifier
        // child sees it. `"object"` is pushed for symmetry / for analyzer
        // helpers that consume the path later.
        let kind = AstKind::JSXMemberExpression(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("object"));
        self.visit_jsx_member_expression_object(&it.object);
        self.key_stack.pop();
        self.key_stack.push(Some("property"));
        self.visit_jsx_identifier(&it.property);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_meta_property(&mut self, it: &MetaProperty<'a>) {
        // In ESTree, `new.target` / `import.meta` are
        // `MetaProperty { meta: Identifier, property: Identifier }`,
        // and both identifiers are read references (resolving to an
        // implicit global) -- `classify_ordinary_reference` produces
        // that classification. oxc keeps them as `IdentifierName`, so
        // push the slot keys here and rely on the dedicated
        // `visit_identifier_name` override below to fire the
        // reference.
        let kind = AstKind::MetaProperty(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("meta"));
        self.visit_identifier_name(&it.meta);
        self.key_stack.pop();
        self.key_stack.push(Some("property"));
        self.visit_identifier_name(&it.property);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_import_attribute(&mut self, it: &ImportAttribute<'a>) {
        // `import x from "y" with { type: "json" }` — in ESTree the
        // attribute key is a plain `Identifier`, producing a read
        // reference for `type` (resolving to an implicit global).
        // Push the slot key so the dedicated `visit_identifier_name`
        // override below can fire it.
        let kind = AstKind::ImportAttribute(self.alloc(it));
        self.enter_node(kind);
        self.visit_span(&it.span);
        self.key_stack.push(Some("key"));
        self.visit_import_attribute_key(&it.key);
        self.key_stack.pop();
        self.key_stack.push(Some("value"));
        self.visit_string_literal(&it.value);
        self.key_stack.pop();
        self.leave_node(kind);
    }

    fn visit_identifier_name(&mut self, it: &IdentifierName<'a>) {
        // oxc keeps `IdentifierName` separate from `IdentifierReference`
        // (ESTree-style `Identifier`); most callers (member-property
        // slots, object-property keys, ...) are non-referential and
        // need no work here. The handful of slots where ESTree emits
        // an `Identifier` that should be classified as a reference are
        // routed in through the `visit_meta_property` /
        // `visit_import_attribute` / `visit_export_specifier`
        // parents, which push the appropriate slot key before
        // reaching this method.
        //
        // ExportSpecifier.local: every `export { X }` /
        // `export { X } from 'src'` shape produces a read reference
        // for that identifier (resolving to an implicit global when
        // the surface form is a re-export from another module). The
        // Rust `oxc_parser` crate distinguishes here:
        // `export { foo }` -> IdentifierReference (already handled by
        // `visit_identifier_reference`), but `export { Lexer } from
        // './Lexer.js'` and `export { default } from 'X'` ->
        // IdentifierName, which reaches this method instead. Fire
        // the reference here so the IR records it.
        if self.type_only_depth == 0 {
            let parent = self.parent_kind();
            let key = self.current_key();
            let route_as_reference = matches!(
                parent,
                Some(AstKind::MetaProperty(_)) | Some(AstKind::ImportAttribute(_))
            ) || (matches!(parent, Some(AstKind::ExportSpecifier(_)))
                && key == Some("local"))
                || (matches!(parent, Some(AstKind::ExportAllDeclaration(_)))
                    && key == Some("exported"));
            if route_as_reference {
                handle_identifier_reference(
                    self.state,
                    self.visitor,
                    parent.as_ref(),
                    key,
                    &self.path,
                    it.name.as_str(),
                    it.span,
                    AstType::Identifier,
                );
            }
        }
        oxc_ast_visit::walk::walk_identifier_name(self, it);
    }

    fn visit_jsx_identifier(&mut self, it: &JSXIdentifier<'a>) {
        // Both `Identifier` and `JSXIdentifier` are routed through
        // the shared reference-creation path; the reference row that
        // gets created (and the def.node on the resulting implicit
        // global) carries the `JSXIdentifier` shape. Route here with
        // `AstType::JSXIdentifier`.
        if self.type_only_depth == 0 {
            let parent = self.parent_kind();
            let key = self.current_key();
            handle_identifier_reference(
                self.state,
                self.visitor,
                parent.as_ref(),
                key,
                &self.path,
                it.name.as_str(),
                it.span,
                AstType::JSXIdentifier,
            );
        }
        oxc_ast_visit::walk::walk_jsx_identifier(self, it);
    }
}

/// `ast_type_of` collapses TypeScript-only Function / MethodDefinition /
/// PropertyDefinition flavours into their value counterparts
/// (`FunctionDeclaration`, `MethodDefinition`, `PropertyDefinition`) so
/// downstream IR consumers see a uniform ESTree shape, but the
/// type-only skip in `enter_node` needs the un-collapsed type
/// (`TSDeclareFunction`, `TSAbstractMethodDefinition`, ...) to decide
/// whether to enter the subtree.
fn ast_type_for_skip(kind: &AstKind<'_>) -> AstType {
    match kind {
        AstKind::Function(f)
            if matches!(f.r#type, oxc_ast::ast::FunctionType::TSDeclareFunction) =>
        {
            AstType::TSDeclareFunction
        }
        _ => ast_type_of(kind),
    }
}

#[cfg(test)]
#[path = "scope_build_visitor_test.rs"]
mod scope_build_visitor_test;
