use super::*;

use crate::language::Language;

#[test]
fn serialized_ir_top_level_field_order() {
    let ir = SerializedIR {
        version: SERIALIZED_IR_VERSION,
        source: SerializedSource {
            path: "input.ts".to_string(),
            language: Language::Ts,
        },
        raw: String::new(),
        scopes: Vec::new(),
        variables: Vec::new(),
        references: Vec::new(),
        unused_variable_ids: Vec::new(),
        diagnostics: Vec::new(),
    };
    let json =
        serde_json::to_string(&ir).expect("SerializedIR serialises to JSON via serde derive");
    assert_eq!(
        json,
        r#"{"version":1,"source":{"path":"input.ts","language":"ts"},"raw":"","scopes":[],"variables":[],"references":[],"unusedVariableIds":[],"diagnostics":[]}"#
    );
}
