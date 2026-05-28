use super::*;

use crate::nesting_kind::NestingDepth;
use crate::primitive::{SourceColumn, SourceLine, Span, Utf16CodeUnitOffset};

fn scope_id(s: &str) -> SerializedScopeId {
    SerializedScopeId::new(s.to_string())
}

#[test]
fn serialized_scope_field_order() {
    let scope = SerializedScope {
        id: scope_id("scope#0"),
        r#type: ScopeType::Module,
        is_strict: true,
        upper: None,
        child_scopes: Vec::new(),
        variable_scope: scope_id("scope#0"),
        block: SerializedBlock {
            r#type: AstType::Program,
            span: Span {
                line: SourceLine(1),
                column: SourceColumn(0),
                offset: Utf16CodeUnitOffset(0),
            },
            end_span: Span {
                line: SourceLine(1),
                column: SourceColumn(0),
                offset: Utf16CodeUnitOffset(0),
            },
        },
        variables: Vec::new(),
        references: Vec::new(),
        through: Vec::new(),
        function_expression_scope: false,
        block_context: None,
        falls_through: false,
        exits_function: false,
        nesting_depths: NestingDepths::uniform(NestingDepth(0)),
    };
    let json =
        serde_json::to_string(&scope).expect("SerializedScope serialises to JSON via serde derive");
    let object_start = json
        .find('{')
        .expect("serialised JSON begins with an object");
    let keys = extract_top_level_keys(&json[object_start..]);
    assert_eq!(
        keys,
        vec![
            "id",
            "type",
            "isStrict",
            "upper",
            "childScopes",
            "variableScope",
            "block",
            "variables",
            "references",
            "through",
            "functionExpressionScope",
            "blockContext",
            "fallsThrough",
            "exitsFunction",
            "nestingDepths",
        ]
    );
}

fn extract_top_level_keys(json: &str) -> Vec<String> {
    let bytes = json.as_bytes();
    assert_eq!(bytes[0], b'{');
    let mut keys = Vec::new();
    let mut i = 1usize;
    let mut depth = 0i32;
    let mut in_string = false;
    let mut string_start = 0usize;
    let mut expect_key = true;
    while i < bytes.len() {
        let c = bytes[i];
        if in_string {
            if c == b'\\' {
                i += 2;
                continue;
            }
            if c == b'"' {
                if depth == 0 && expect_key {
                    let key = std::str::from_utf8(&bytes[string_start + 1..i])
                        .expect("slice carved from a UTF-8 source string is still UTF-8")
                        .to_string();
                    keys.push(key);
                    expect_key = false;
                }
                in_string = false;
            }
            i += 1;
            continue;
        }
        match c {
            b'"' => {
                in_string = true;
                string_start = i;
            }
            b'{' | b'[' => depth += 1,
            b'}' | b']' => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
            }
            b',' if depth == 0 => expect_key = true,
            _ => {}
        }
        i += 1;
    }
    keys
}
