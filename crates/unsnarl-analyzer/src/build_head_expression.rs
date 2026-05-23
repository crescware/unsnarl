//! Reduce an ExpressionStatement's `expression` to the
//! `HeadExpression` mini-AST.
//!
//! Matches on `oxc_ast::Expression` variants directly. Unrecognised
//! shapes fall back to `HeadExpression::Raw` with the surrounding
//! ExpressionStatement's span (passed as `fallback`).

use oxc_ast::ast::Expression;
use oxc_span::{GetSpan, Span};
use oxc_syntax::operator::{
    AssignmentOperator as OxcAssignmentOperator, UpdateOperator as OxcUpdateOperator,
};

use unsnarl_ir::reference::{HeadExpression, HeadOperand};
use unsnarl_ir::Utf8ByteOffset;
use unsnarl_oxc_parity::{AssignOperator, UpdateOperator};

/// Build a `HeadExpression` for an ExpressionStatement whose
/// `expression` is `expression` (or `None` for an empty / synthesised
/// statement). `fallback` is the span the emitter slices when the
/// expression doesn't reduce to a structural head.
pub fn build_head_expression(
    expression: Option<&Expression<'_>>,
    fallback: Span,
) -> HeadExpression {
    let from_structure = expression.and_then(try_build);
    from_structure.unwrap_or_else(|| raw_from_expression(expression, fallback))
}

fn try_build(node: &Expression<'_>) -> Option<HeadExpression> {
    match node {
        Expression::Identifier(id) => {
            let name = id.name.as_str();
            if name.is_empty() {
                return None;
            }
            Some(HeadExpression::identifier(name.to_string()))
        }
        Expression::StaticMemberExpression(m) => {
            let object_head = try_build(&m.object)?;
            let prop = m.property.name.as_str();
            if prop.is_empty() {
                return None;
            }
            Some(HeadExpression::member(object_head, prop.to_string()))
        }
        Expression::ComputedMemberExpression(_) => None,
        Expression::PrivateFieldExpression(m) => {
            // The head expression for `obj.#prop` must come out as
            // `{ kind: member, object, property: "prop" }` -- the
            // same shape as a non-private `obj.prop`. The Rust
            // `oxc_parser` crate keeps `PrivateFieldExpression`
            // separate, so flatten it onto a member head here using
            // the bare private name.
            let object_head = try_build(&m.object)?;
            let prop = m.field.name.as_str();
            if prop.is_empty() {
                return None;
            }
            Some(HeadExpression::member(object_head, prop.to_string()))
        }
        Expression::CallExpression(call) => {
            let callee_head = try_build_callee(&call.callee)?;
            Some(HeadExpression::Call {
                callee: Box::new(callee_head),
            })
        }
        Expression::NewExpression(new_expr) => {
            let callee_head = try_build_callee(&new_expr.callee)?;
            Some(HeadExpression::New {
                callee: Box::new(callee_head),
            })
        }
        Expression::AwaitExpression(aw) => {
            let arg_head = try_build(&aw.argument)?;
            Some(HeadExpression::Await {
                argument: Box::new(arg_head),
            })
        }
        Expression::AssignmentExpression(asn) => {
            let operator = convert_assign_operator(asn.operator);
            let left_target_head = try_build_assignment_target(&asn.left);
            let right_head = try_build(&asn.right);
            let left_operand = operand_from(left_target_span(&asn.left), left_target_head);
            let right_operand = operand_from(asn.right.span(), right_head);
            // Both sides collapsed to `elided` — fall back to raw.
            if matches!(left_operand.head, HeadExpression::Elided)
                && matches!(right_operand.head, HeadExpression::Elided)
            {
                return None;
            }
            Some(HeadExpression::Assign {
                operator,
                left: Box::new(left_operand),
                right: Box::new(right_operand),
            })
        }
        Expression::UpdateExpression(up) => {
            let operator = convert_update_operator(up.operator);
            // `argument` is a `SimpleAssignmentTarget`; try to reduce
            // it via its expression form. If the structural reduction
            // fails the whole UpdateExpression is unrecognised (head
            // collapses to raw).
            let arg_head = try_build_simple_assignment_target(&up.argument)?;
            let arg_operand =
                operand_from(simple_assignment_target_span(&up.argument), Some(arg_head));
            Some(HeadExpression::Update {
                operator,
                prefix: up.prefix,
                argument: Box::new(arg_operand),
            })
        }
        _ => None,
    }
}

fn try_build_callee<'a>(callee: &Expression<'a>) -> Option<HeadExpression> {
    try_build(callee)
}

fn try_build_assignment_target(
    target: &oxc_ast::ast::AssignmentTarget<'_>,
) -> Option<HeadExpression> {
    use oxc_ast::ast::AssignmentTarget as AT;
    match target {
        AT::AssignmentTargetIdentifier(id) => {
            let name = id.name.as_str();
            if name.is_empty() {
                return None;
            }
            Some(HeadExpression::identifier(name.to_string()))
        }
        AT::StaticMemberExpression(m) => {
            let object_head = try_build(&m.object)?;
            let prop = m.property.name.as_str();
            if prop.is_empty() {
                return None;
            }
            Some(HeadExpression::member(object_head, prop.to_string()))
        }
        AT::PrivateFieldExpression(m) => {
            // Same flattening as the expression-position arm:
            // `obj.#prop = rhs` reduces to a member head with the
            // bare private name in the property slot, and the head
            // IR emits the surrounding assignment shape.
            let object_head = try_build(&m.object)?;
            let prop = m.field.name.as_str();
            if prop.is_empty() {
                return None;
            }
            Some(HeadExpression::member(object_head, prop.to_string()))
        }
        AT::ComputedMemberExpression(_)
        | AT::TSAsExpression(_)
        | AT::TSSatisfiesExpression(_)
        | AT::TSNonNullExpression(_)
        | AT::TSTypeAssertion(_) => None,
        _ => None,
    }
}

fn try_build_simple_assignment_target(
    target: &oxc_ast::ast::SimpleAssignmentTarget<'_>,
) -> Option<HeadExpression> {
    use oxc_ast::ast::SimpleAssignmentTarget as SAT;
    match target {
        SAT::AssignmentTargetIdentifier(id) => {
            let name = id.name.as_str();
            if name.is_empty() {
                return None;
            }
            Some(HeadExpression::identifier(name.to_string()))
        }
        SAT::StaticMemberExpression(m) => {
            let object_head = try_build(&m.object)?;
            let prop = m.property.name.as_str();
            if prop.is_empty() {
                return None;
            }
            Some(HeadExpression::member(object_head, prop.to_string()))
        }
        SAT::PrivateFieldExpression(m) => {
            // Same flattening as the AssignmentTarget arm above:
            // an UpdateExpression's argument is a
            // SimpleAssignmentTarget, and `++obj.#prop` reduces to a
            // member head with the bare private name.
            let object_head = try_build(&m.object)?;
            let prop = m.field.name.as_str();
            if prop.is_empty() {
                return None;
            }
            Some(HeadExpression::member(object_head, prop.to_string()))
        }
        SAT::ComputedMemberExpression(_)
        | SAT::TSAsExpression(_)
        | SAT::TSSatisfiesExpression(_)
        | SAT::TSNonNullExpression(_)
        | SAT::TSTypeAssertion(_) => None,
    }
}

fn left_target_span(target: &oxc_ast::ast::AssignmentTarget<'_>) -> Span {
    target.span()
}

fn simple_assignment_target_span(target: &oxc_ast::ast::SimpleAssignmentTarget<'_>) -> Span {
    use oxc_ast::ast::SimpleAssignmentTarget as SAT;
    match target {
        SAT::AssignmentTargetIdentifier(id) => id.span,
        SAT::StaticMemberExpression(m) => m.span,
        SAT::ComputedMemberExpression(m) => m.span,
        SAT::PrivateFieldExpression(m) => m.span,
        SAT::TSAsExpression(e) => e.span,
        SAT::TSSatisfiesExpression(e) => e.span,
        SAT::TSNonNullExpression(e) => e.span,
        SAT::TSTypeAssertion(e) => e.span,
    }
}

fn operand_from(span: Span, head: Option<HeadExpression>) -> HeadOperand {
    HeadOperand {
        head: head.unwrap_or(HeadExpression::Elided),
        start_offset: Utf8ByteOffset(span.start),
        end_offset: Utf8ByteOffset(span.end),
    }
}

fn raw_from_expression(expression: Option<&Expression<'_>>, fallback: Span) -> HeadExpression {
    let span = expression.map(|e| e.span()).unwrap_or(fallback);
    HeadExpression::Raw {
        start_offset: Utf8ByteOffset(span.start),
        end_offset: Utf8ByteOffset(span.end),
    }
}

fn convert_assign_operator(op: OxcAssignmentOperator) -> AssignOperator {
    match op {
        OxcAssignmentOperator::Assign => AssignOperator::Assign,
        OxcAssignmentOperator::Addition => AssignOperator::AddAssign,
        OxcAssignmentOperator::Subtraction => AssignOperator::SubAssign,
        OxcAssignmentOperator::Multiplication => AssignOperator::MulAssign,
        OxcAssignmentOperator::Division => AssignOperator::DivAssign,
        OxcAssignmentOperator::Remainder => AssignOperator::RemAssign,
        OxcAssignmentOperator::Exponential => AssignOperator::ExpAssign,
        OxcAssignmentOperator::ShiftLeft => AssignOperator::ShlAssign,
        OxcAssignmentOperator::ShiftRight => AssignOperator::ShrAssign,
        OxcAssignmentOperator::ShiftRightZeroFill => AssignOperator::UnsignedShrAssign,
        OxcAssignmentOperator::BitwiseOR => AssignOperator::BitOrAssign,
        OxcAssignmentOperator::BitwiseXOR => AssignOperator::BitXorAssign,
        OxcAssignmentOperator::BitwiseAnd => AssignOperator::BitAndAssign,
        OxcAssignmentOperator::LogicalOr => AssignOperator::LogicalOrAssign,
        OxcAssignmentOperator::LogicalAnd => AssignOperator::LogicalAndAssign,
        OxcAssignmentOperator::LogicalNullish => AssignOperator::NullishAssign,
    }
}

fn convert_update_operator(op: OxcUpdateOperator) -> UpdateOperator {
    match op {
        OxcUpdateOperator::Increment => UpdateOperator::Increment,
        OxcUpdateOperator::Decrement => UpdateOperator::Decrement,
    }
}

#[cfg(test)]
#[path = "build_head_expression_test.rs"]
mod build_head_expression_test;
