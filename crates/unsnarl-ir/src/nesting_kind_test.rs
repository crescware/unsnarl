use super::*;

#[test]
fn nesting_depths_emits_keys_in_declared_order() {
    let depths = NestingDepths::uniform(NestingDepth(0));
    let json = serde_json::to_string(&depths).expect("value serialises to JSON via serde derive");
    assert_eq!(
        json,
        r#"{"function":0,"if":0,"for":0,"while":0,"switch":0,"try-catch-finally":0,"block":0}"#
    );
}

#[test]
fn nesting_kind_serializes_kebab_case() {
    assert_eq!(
        serde_json::to_string(&NestingKind::Function)
            .expect("value serialises to JSON via serde derive"),
        r#""function""#
    );
    assert_eq!(
        serde_json::to_string(&NestingKind::TryCatchFinally)
            .expect("value serialises to JSON via serde derive"),
        r#""try-catch-finally""#
    );
}
