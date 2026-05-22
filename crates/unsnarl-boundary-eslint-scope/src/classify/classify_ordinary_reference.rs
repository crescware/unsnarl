//! Classify an identifier reference that isn't in a binding /
//! skip slot.
//!
//! Matches on `AstKind::AssignmentExpression` and reads
//! `AssignmentExpression.operator: AssignmentOperator` directly to
//! pick the `WRITE` vs `READ_WRITE` flag combination.

use oxc_ast::AstKind;
use oxc_syntax::operator::AssignmentOperator;

use unsnarl_ir::reference::reference_flags::ReferenceFlags;

use crate::classify::classify_result::ClassifyResult;
use crate::classify::reference::reference;

pub(crate) fn classify_ordinary_reference(
    parent: &AstKind<'_>,
    key: Option<&'static str>,
) -> ClassifyResult {
    if let AstKind::AssignmentExpression(ae) = parent {
        if key == Some("left") {
            let flags = if matches!(ae.operator, AssignmentOperator::Assign) {
                ReferenceFlags::WRITE
            } else {
                ReferenceFlags::READ | ReferenceFlags::WRITE
            };
            return reference(flags, false);
        }
    }
    if matches!(parent, AstKind::UpdateExpression(_)) && key == Some("argument") {
        return reference(ReferenceFlags::READ | ReferenceFlags::WRITE, false);
    }
    let init = matches!(parent, AstKind::VariableDeclarator(_)) && key == Some("init");
    reference(ReferenceFlags::READ, init)
}

#[cfg(test)]
#[path = "classify_ordinary_reference_test.rs"]
mod classify_ordinary_reference_test;
