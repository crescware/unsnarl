//! Sibling tests for [`edge_label_of_ref`]. Cases lock the
//! read / write / call / receiver flag combinations.

use super::edge_label_of_ref;
use unsnarl_ir::primitive::Span;
use unsnarl_ir::serialized::{
    SerializedCompletion, SerializedFlags, SerializedReference, SerializedReferenceId,
    SerializedReferenceIdentifier, SerializedScopeId,
};

fn ref_with_flags(read: bool, write: bool, call: bool) -> SerializedReference {
    SerializedReference {
        id: SerializedReferenceId::new("ref_1".to_string()),
        identifier: SerializedReferenceIdentifier::new(
            "x".to_string(),
            Span {
                line: unsnarl_ir::primitive::SourceLine(1),
                column: unsnarl_ir::primitive::SourceColumn(0),
                offset: unsnarl_ir::primitive::SourceOffset(0),
            },
        ),
        from: SerializedScopeId::new("scope_0".to_string()),
        resolved: None,
        owners: Vec::new(),
        init: false,
        flags: SerializedFlags {
            read,
            write,
            call,
            receiver: false,
        },
        predicate_container: None,
        completion: SerializedCompletion::Normal,
        jsx_element: None,
        expression_statement_container: None,
    }
}

#[test]
fn read_only() {
    assert_eq!(
        edge_label_of_ref(&ref_with_flags(true, false, false)),
        "read"
    );
}

#[test]
fn write_only() {
    assert_eq!(
        edge_label_of_ref(&ref_with_flags(false, true, false)),
        "write"
    );
}

#[test]
fn call_only() {
    assert_eq!(
        edge_label_of_ref(&ref_with_flags(false, false, true)),
        "call"
    );
}

#[test]
fn read_and_write_joined_with_comma() {
    assert_eq!(
        edge_label_of_ref(&ref_with_flags(true, true, false)),
        "read,write"
    );
}

#[test]
fn read_and_call_joined_with_comma() {
    assert_eq!(
        edge_label_of_ref(&ref_with_flags(true, false, true)),
        "read,call"
    );
}

#[test]
fn all_three_joined_with_comma_in_canonical_order() {
    assert_eq!(
        edge_label_of_ref(&ref_with_flags(true, true, true)),
        "read,write,call"
    );
}

#[test]
fn no_flags_falls_back_to_ref() {
    assert_eq!(
        edge_label_of_ref(&ref_with_flags(false, false, false)),
        "ref"
    );
}
