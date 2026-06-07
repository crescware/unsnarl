//! `fire_reference`: fill the `ReferenceAnnotation` row for an
//! identifier-shaped node whose span matches an arena reference.

use oxc_ast::AstKind;
use oxc_span::Span;

use unsnarl_annotations::ReferenceAnnotation;
use unsnarl_ir::primitive::Utf8ByteOffset;

use crate::expression_statement_container::build_expression_statement_container;
use crate::find_completion::find_completion;
use crate::find_jsx_element_span::find_jsx_element_span;
use crate::find_predicate_container::find_predicate_container;
use crate::owner::{
    all_binding_variables, locate_reference_owner_slot, walk_assignment_target_identifiers,
    OwnerLookup,
};
use crate::reference_call_receiver::reference_call_receiver_flags;

use super::{BuildAnalysisVisitor, PathFrame};

impl<'a, 'arena> BuildAnalysisVisitor<'a, 'arena> {
    /// Fill the `ReferenceAnnotation` row for an identifier-shaped
    /// node (`IdentifierReference`, `BindingIdentifier`, or
    /// `JSXIdentifier`) whose span matches an entry in `span_to_ref`.
    pub(super) fn fire_reference(&mut self, span: Span) {
        let Some(&ref_id) = self.span_to_ref.get(&(span.start, span.end)) else {
            return;
        };
        let scope = self.arena.references[ref_id].from;
        let parent_node = self.parent_ast_node();
        let parent_type = parent_node.map(|n| n.r#type.clone());
        let parent_offset = parent_node.map(|n| Utf8ByteOffset(n.span.start));
        let key = self.current_key();
        let owners = match locate_reference_owner_slot(&self.path_entries) {
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
                        // For AssignmentExpression targets, owners must
                        // resolve as they would inline with scope-build,
                        // i.e. against `scope.set` at the moment the
                        // inner reference fires. Analysis runs as a
                        // separate pass after `scope.set` is fully
                        // populated, so a fresh `resolve_in_scope_chain`
                        // call here would pick up `var` bindings
                        // hoisted from later sibling blocks that
                        // weren't visible at binding time. Reuse each
                        // identifier's `reference.resolved` value
                        // instead.
                        let mut out = Vec::new();
                        walk_assignment_target_identifiers(&ae.left, &mut |id| {
                            let Some(&ident_ref_id) =
                                self.span_to_ref.get(&(id.span.start, id.span.end))
                            else {
                                return;
                            };
                            if let Some(resolved) = self.arena.references[ident_ref_id].resolved {
                                if !out.contains(&resolved) {
                                    out.push(resolved);
                                }
                            }
                        });
                        out
                    }
                    _ => Vec::new(),
                }
            }
        };
        let flags = reference_call_receiver_flags(parent_type.as_ref(), key);
        let predicate_container = find_predicate_container(
            parent_type.as_ref(),
            parent_offset,
            key,
            &self.path_entries,
            self.index,
        );
        let completion = find_completion(&self.path_entries);
        let jsx_element = find_jsx_element_span(&self.path_entries);
        let expression_statement_container =
            self.path
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, f)| match &f.kind {
                    AstKind::ExpressionStatement(es) => {
                        if is_synthetic_arrow_body_expression_statement(&self.path, i) {
                            None
                        } else if matches!(
                            es.expression,
                            oxc_ast::ast::Expression::ConditionalExpression(_)
                        ) {
                            // A ternary `cond ? a : b;` renders as the
                            // `ternary ?:` diamond plus `? then` / `: else`
                            // branch subgraphs (an arm being a synthetic
                            // scope). Emitting an ExpressionStatement
                            // container too would duplicate the whole
                            // statement as a verbatim Raw head node and
                            // pull any arm-local callback into a
                            // statement-level CallProxy instead of the
                            // arm. Suppress it so the structure stands
                            // alone — the arm values flow to their
                            // consumer exactly as in any other context.
                            None
                        } else {
                            Some(build_expression_statement_container(
                                es.span,
                                Some(&es.expression),
                            ))
                        }
                    }
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
}

/// oxc represents the body of an expression-arrow (`() => expr`) as
/// a `FunctionBody { statements: [ExpressionStatement { expression:
/// expr }] }` synthetic wrapper, while the ESTree shape keeps
/// `ArrowFunctionExpression.body = Expression` directly. The IR
/// emitter must NOT report that synthetic ExpressionStatement as the
/// reference's `expressionStatementContainer`, so flag it here based
/// on its path position: ExpressionStatement at index `i` is
/// synthetic iff `path[i-1]` is a FunctionBody and `path[i-2]` is an
/// ArrowFunctionExpression whose `arrow_body.is_block == false`.
fn is_synthetic_arrow_body_expression_statement(path: &[PathFrame<'_>], i: usize) -> bool {
    let Some(prev) = i.checked_sub(1).and_then(|j| path.get(j)) else {
        return false;
    };
    if !matches!(prev.kind, AstKind::FunctionBody(_)) {
        return false;
    }
    let Some(arrow) = i.checked_sub(2).and_then(|j| path.get(j)) else {
        return false;
    };
    if !matches!(arrow.kind, AstKind::ArrowFunctionExpression(_)) {
        return false;
    }
    arrow
        .arrow_body
        .as_ref()
        .map(|b| !b.is_block)
        .unwrap_or(false)
}
