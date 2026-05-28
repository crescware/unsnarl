use unsnarl_ir::reference::ReferenceCompletion;

use super::*;

#[test]
fn flags_emits_keys_in_declared_order() {
    let flags = ReferenceAnnotationFlags {
        call: false,
        receiver: false,
    };
    let json = serde_json::to_string(&flags).expect("ReferenceAnnotationFlags serialises to JSON");
    assert_eq!(json, r#"{"call":false,"receiver":false}"#);
}

#[test]
fn can_construct_with_all_fields() {
    let ann = ReferenceAnnotation {
        owners: Vec::new(),
        flags: ReferenceAnnotationFlags {
            call: false,
            receiver: false,
        },
        predicate_container: None,
        completion: ReferenceCompletion::Normal,
        jsx_element: None,
        expression_statement_container: None,
    };

    assert!(ann.owners.is_empty());
    assert!(!ann.flags.call);
    assert!(!ann.flags.receiver);
    assert!(ann.predicate_container.is_none());
    assert!(matches!(ann.completion, ReferenceCompletion::Normal));
    assert!(ann.jsx_element.is_none());
    assert!(ann.expression_statement_container.is_none());
}
