use unsnarl_oxc_parity::AstType;

use super::reference_call_receiver_flags;

#[test]
fn call_expression_callee_marks_call_only() {
    let f = reference_call_receiver_flags(Some(&AstType::CallExpression), Some("callee"));
    assert!(f.call);
    assert!(!f.receiver);
}

#[test]
fn new_expression_callee_marks_call_only() {
    let f = reference_call_receiver_flags(Some(&AstType::NewExpression), Some("callee"));
    assert!(f.call);
    assert!(!f.receiver);
}

#[test]
fn member_expression_object_marks_receiver_only() {
    let f = reference_call_receiver_flags(Some(&AstType::MemberExpression), Some("object"));
    assert!(!f.call);
    assert!(f.receiver);
}

#[test]
fn call_expression_non_callee_is_neither() {
    let f = reference_call_receiver_flags(Some(&AstType::CallExpression), Some("arguments"));
    assert!(!f.call);
    assert!(!f.receiver);
}

#[test]
fn member_expression_non_object_is_neither() {
    let f = reference_call_receiver_flags(Some(&AstType::MemberExpression), Some("property"));
    assert!(!f.call);
    assert!(!f.receiver);
}

#[test]
fn null_parent_is_neither() {
    let f = reference_call_receiver_flags(None, None);
    assert!(!f.call);
    assert!(!f.receiver);
}

#[test]
fn unrelated_parent_type_is_neither() {
    let f = reference_call_receiver_flags(Some(&AstType::VariableDeclarator), Some("init"));
    assert!(!f.call);
    assert!(!f.receiver);
}
