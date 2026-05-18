use super::*;

use crate::ast_type::AstType;
use crate::filled_string::FilledString;
use crate::primitive::Span;
use crate::variable_declaration_kind::VariableDeclarationKind;

fn filled(s: &str) -> FilledString {
    FilledString::new(s).unwrap()
}

fn name() -> DefinitionName {
    DefinitionName {
        name: filled("x"),
        span: Span {
            line: 1,
            column: 0,
            offset: 0,
        },
    }
}

fn node(r#type: AstType) -> DefinitionNode {
    DefinitionNode {
        r#type,
        span: Span {
            line: 1,
            column: 0,
            offset: 0,
        },
    }
}

#[test]
fn variable_def_field_order_matches_ts() {
    let def = VariableDef::new(
        name(),
        node(AstType::VariableDeclarator),
        Some(node(AstType::VariableDeclaration)),
        Some(node(AstType::Literal)),
        VariableDeclarationKind::Const,
    );
    let json = serde_json::to_string(&def).unwrap();
    let object_start = json.find('{').unwrap();
    let keys = extract_top_level_keys(&json[object_start..]);
    assert_eq!(
        keys,
        vec!["name", "node", "parent", "type", "init", "declarationKind"]
    );
    assert!(json.contains(r#""type":"Variable""#));
}

#[test]
fn import_binding_named_field_order_matches_ts() {
    let def = ImportBindingNamedDef::new(
        name(),
        node(AstType::ImportSpecifier),
        Some(node(AstType::ImportDeclaration)),
        filled("Sub"),
        filled("./sub"),
    );
    let json = serde_json::to_string(&def).unwrap();
    let object_start = json.find('{').unwrap();
    let keys = extract_top_level_keys(&json[object_start..]);
    assert_eq!(
        keys,
        vec![
            "name",
            "node",
            "parent",
            "type",
            "importKind",
            "importedName",
            "importSource",
        ]
    );
    assert!(json.contains(r#""type":"ImportBinding""#));
    assert!(json.contains(r#""importKind":"named""#));
}

#[test]
fn import_binding_default_field_order_matches_ts() {
    let def = ImportBindingDefaultDef::new(
        name(),
        node(AstType::ImportDefaultSpecifier),
        Some(node(AstType::ImportDeclaration)),
        filled("./sub"),
    );
    let json = serde_json::to_string(&def).unwrap();
    let object_start = json.find('{').unwrap();
    let keys = extract_top_level_keys(&json[object_start..]);
    assert_eq!(
        keys,
        vec![
            "name",
            "node",
            "parent",
            "type",
            "importKind",
            "importSource"
        ]
    );
    assert!(json.contains(r#""importKind":"default""#));
}

#[test]
fn simple_def_field_order_matches_ts() {
    let def = SimpleDef {
        name: name(),
        node: node(AstType::FunctionDeclaration),
        parent: None,
        r#type: SimpleDefType::FunctionName,
    };
    let json = serde_json::to_string(&def).unwrap();
    let object_start = json.find('{').unwrap();
    let keys = extract_top_level_keys(&json[object_start..]);
    assert_eq!(keys, vec!["name", "node", "parent", "type"]);
    assert!(json.contains(r#""type":"FunctionName""#));
}

/// Walks the JSON object, ignoring nested braces / brackets / strings,
/// and returns the keys at the top level in declaration order. Avoids
/// pulling a JSON tokenizer just for this test, while still being
/// robust against nested objects in the values.
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
                        .unwrap()
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
