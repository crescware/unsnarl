use super::*;

#[test]
fn emits_keys_in_declared_order() {
    let ann = VariableAnnotation { is_unused: false };
    let json = serde_json::to_string(&ann).expect("VariableAnnotation serialises to JSON");
    assert_eq!(json, r#"{"isUnused":false}"#);
}
