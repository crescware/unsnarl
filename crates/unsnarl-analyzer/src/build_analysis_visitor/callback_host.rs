//! Callback-host resolution: build the `CallbackArgument` /
//! `CallbackHost` annotations that pair a callback's enclosing call
//! with the binding / return / assignment that consumes its result.

use oxc_ast::ast::{AssignmentTarget, Expression};
use oxc_ast::AstKind;
use oxc_span::{GetSpan, Span};

use unsnarl_ir::primitive::{AstNode, Utf8ByteOffset};
use unsnarl_ir::scope::{CallbackArgument, CallbackHost, CallbackHostKind};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_oxc_parity::AstType;

use crate::build_head_expression::{build_callee_head, build_head_expression};

use super::BuildAnalysisVisitor;

impl<'a, 'arena> BuildAnalysisVisitor<'a, 'arena> {
    /// Build a [`CallbackArgument`] annotation when a function scope
    /// is the `arg_index`-th argument of an enclosing
    /// `CallExpression` / `NewExpression`. Returns `None` otherwise.
    ///
    /// The annotation captures only the structural fact: the enclosing
    /// call's `callee` head subtree and the `arg_index`. Whether the
    /// callback is hosted by a statement-level CallProxy wrapper is a
    /// visual-graph rendering concern resolved there, so no statement
    /// position is recorded here.
    pub(super) fn callback_argument_for(
        &self,
        scope_type: ScopeType,
        parent_node: Option<&AstNode>,
    ) -> Option<CallbackArgument> {
        if !matches!(scope_type, ScopeType::Function) {
            return None;
        }
        let parent = parent_node?;
        if !matches!(
            parent.r#type,
            AstType::CallExpression | AstType::NewExpression
        ) {
            return None;
        }
        // The function must sit in the parent call's `arguments`
        // slot. Without this guard the function scope of a callee
        // (e.g. the IIFE in `outer((function(){})())`) would inherit
        // the outer call's `arg_index_stack` top and be misannotated
        // as `outer`'s arg.
        if self.current_key() != Some("arguments") {
            return None;
        }
        let arg_index = self.current_arg_index()? as u32;
        // The parent path frame is the enclosing call; read its
        // `callee` subtree directly so the label does not depend on a
        // surrounding `ExpressionStatement` head being present.
        let (callee, call_span) = match self.path.last().map(|f| &f.kind) {
            Some(AstKind::CallExpression(call)) => (&call.callee, call.span),
            Some(AstKind::NewExpression(new_expr)) => (&new_expr.callee, new_expr.span),
            _ => return None,
        };
        let callee_head = build_callee_head(callee, callee.span());
        Some(CallbackArgument::new(
            callee_head,
            arg_index,
            self.callback_host(call_span),
        ))
    }

    /// Find the binding / return / assignment whose value is the
    /// callback's enclosing call, walking the path outward.
    ///
    /// Yields `None` at an `ExpressionStatement` (those callbacks are
    /// recovered downstream from the statement spans) and, for an
    /// `AssignmentExpression`, only when the callback's call is the
    /// assignment's *value* rather than its target. `callback_call_span`
    /// (the enclosing call's span) drives that value-vs-target check.
    pub(super) fn callback_host(&self, callback_call_span: Span) -> Option<CallbackHost> {
        for frame in self.path.iter().rev() {
            match &frame.kind {
                AstKind::VariableDeclarator(vd) => {
                    // The declarator init's span is the bound expression
                    // and matches the variable's serialized init offset,
                    // so the visual layer can pair them.
                    return vd.init.as_ref().map(|init| {
                        let head = head_source_for_call(init, callback_call_span);
                        self.build_callback_host(
                            CallbackHostKind::VariableDeclarator,
                            init.span(),
                            head,
                            None,
                        )
                    });
                }
                AstKind::ReturnStatement(rs) => {
                    // Span the whole `return ...;` so it matches the
                    // reference completion span (`find_completion` keys on
                    // the ReturnStatement node), letting the visual layer
                    // route the returned call's inputs to this proxy. The
                    // label still comes from the returned expression.
                    return rs.argument.as_ref().map(|arg| {
                        let head = head_source_for_call(arg, callback_call_span);
                        self.build_callback_host(CallbackHostKind::Return, rs.span(), head, None)
                    });
                }
                AstKind::AssignmentExpression(ae) => {
                    // Host the callback only when its call is in the
                    // assignment's value (`ae.right`). A callback in the
                    // *target* on the left -- a computed-member key or a
                    // destructuring default, e.g. `obj[arr.map(cb)] = x` --
                    // is not the assignment's value, so it gets no host and
                    // is left to the statement / island path. Otherwise the
                    // host would describe the unrelated `ae.right`.
                    let rhs = ae.right.span();
                    if callback_call_span.start < rhs.start || callback_call_span.end > rhs.end {
                        return None;
                    }
                    // Record the target identifier's offset only for a
                    // plain-identifier LHS (`y = arr.map(cb)`); member /
                    // destructuring targets have no single target
                    // identifier, so they stay `None`.
                    let target_offset = match &ae.left {
                        AssignmentTarget::AssignmentTargetIdentifier(id) => {
                            Some(Utf8ByteOffset(id.span.start))
                        }
                        _ => None,
                    };
                    let head = head_source_for_call(&ae.right, callback_call_span);
                    return Some(self.build_callback_host(
                        CallbackHostKind::Assignment,
                        rhs,
                        head,
                        target_offset,
                    ));
                }
                AstKind::ArrowFunctionExpression(arrow) => {
                    // An expression-body arrow (`(x) => expr`) implicitly
                    // returns its body, so a callback inside that body is
                    // hosted by the body expression -- the same span
                    // `find_completion` reports for the body's references,
                    // so the visual layer can route them to the proxy.
                    // A block-body arrow is a function boundary: any host
                    // would have been hit deeper, so stop.
                    return match frame.arrow_body {
                        Some(b) if !b.is_block => arrow.get_expression().map(|expr| {
                            let head = head_source_for_call(expr, callback_call_span);
                            self.build_callback_host(CallbackHostKind::Return, b.span, head, None)
                        }),
                        _ => None,
                    };
                }
                // Other function / class boundaries: the callback is
                // inside another function whose own host would have been
                // hit earlier, so do not cross out.
                AstKind::Function(_) | AstKind::Class(_) => return None,
                AstKind::ExpressionStatement(_) => return None,
                _ => {}
            }
        }
        None
    }

    pub(super) fn build_callback_host(
        &self,
        kind: CallbackHostKind,
        span: Span,
        head_expr: &Expression<'_>,
        target_offset: Option<Utf8ByteOffset>,
    ) -> CallbackHost {
        CallbackHost {
            kind,
            start_offset: Utf8ByteOffset(span.start),
            end_offset: Utf8ByteOffset(span.end),
            head: build_head_expression(Some(head_expr), head_expr.span()),
            target_offset,
        }
    }
}

/// The expression whose head should label the callback's CallProxy.
///
/// Normally this is the host's whole bound expression (the declarator
/// init / returned / assigned value). But when that value is a ternary
/// `cond ? a : b`, the callback lives in exactly one arm and its value
/// reaches the host *through* that arm — so the proxy must stand for the
/// arm's own call (`items.map()`), not the whole `cond ? … : …` text,
/// which would otherwise render as a raw head duplicating the
/// `ternary ?:` container. Descend into the arm that contains the call
/// (recursing for nested ternaries); the host span itself is kept by the
/// caller so the result-variable pairing (by init offset) is unaffected.
/// A call in the ternary's `test` matches no arm and falls back to the
/// whole expression.
fn head_source_for_call<'b, 'a>(bound: &'b Expression<'a>, call_span: Span) -> &'b Expression<'a> {
    if let Expression::ConditionalExpression(cond) = bound {
        for arm in [&cond.consequent, &cond.alternate] {
            let s = arm.span();
            if s.start <= call_span.start && call_span.end <= s.end {
                return head_source_for_call(arm, call_span);
            }
        }
    }
    bound
}

#[cfg(test)]
#[path = "callback_host_test.rs"]
mod callback_host_test;
