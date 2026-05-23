use oxc_index::IndexVec;
use oxc_span::Span;

use unsnarl_ir::primitive::{AstIdentifier, SourceIndex, Utf16CodeUnitOffset};
use unsnarl_ir::scope::{DefinitionData, VariableData};
use unsnarl_ir::{DefinitionType, IrArena, ScopeId};
use unsnarl_oxc_parity::AstType;

use super::pick_variable_offset;

fn empty_arena() -> IrArena {
    IrArena {
        scopes: IndexVec::new(),
        variables: IndexVec::new(),
        references: IndexVec::new(),
        definitions: IndexVec::new(),
    }
}

fn placeholder_scope_id() -> ScopeId {
    ScopeId::from_usize(0)
}

fn ident_at(name: &str, byte_start: u32) -> AstIdentifier {
    AstIdentifier::new(
        AstType::Identifier,
        name.to_string(),
        Span::new(byte_start, byte_start + (name.len() as u32)),
    )
}

#[test]
fn ascii_source_returns_byte_offset_unchanged() {
    // No non-ASCII bytes in `raw`, so UTF-16 offset == UTF-8 byte offset.
    let raw = "const x = 1;\n";
    let mut arena = empty_arena();
    let head = ident_at("x", 6);
    let var_id = arena.variables.push(VariableData::new(
        "x".to_string(),
        placeholder_scope_id(),
        vec![head],
        Vec::new(),
        Vec::new(),
    ));
    assert_eq!(
        pick_variable_offset(&arena, var_id, &SourceIndex::build(raw)),
        Utf16CodeUnitOffset(6)
    );
}

#[test]
fn non_ascii_before_head_identifier_shifts_offset_to_utf16_units() {
    // Em-dash `—` is 3 UTF-8 bytes / 1 UTF-16 code unit. With one
    // em-dash in the leading comment, the byte offset of `x` is 13
    // but its UTF-16 offset is 11; `pick_variable_offset` must
    // report the latter to honour the IR's UTF-16 offset contract.
    let raw = "// —\nconst x = 1;\n";
    let mut arena = empty_arena();
    let head = ident_at("x", 13);
    let var_id = arena.variables.push(VariableData::new(
        "x".to_string(),
        placeholder_scope_id(),
        vec![head],
        Vec::new(),
        Vec::new(),
    ));
    assert_eq!(
        pick_variable_offset(&arena, var_id, &SourceIndex::build(raw)),
        Utf16CodeUnitOffset(11)
    );
}

#[test]
fn falls_back_to_first_def_name_when_identifiers_is_empty() {
    // Same em-dash scenario, but the variable has no `identifiers`
    // entry. The fallback path through `defs[0].name.span.start` must
    // also convert byte → UTF-16.
    let raw = "// —\nconst x = 1;\n";
    let mut arena = empty_arena();
    let def_name = ident_at("x", 13);
    let def_node = unsnarl_ir::primitive::AstNode {
        r#type: AstType::VariableDeclarator,
        span: Span::new(13, 18),
    };
    let def_id = arena.definitions.push(DefinitionData {
        r#type: DefinitionType::Variable,
        name: def_name,
        node: def_node,
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    });
    let var_id = arena.variables.push(VariableData::new(
        "x".to_string(),
        placeholder_scope_id(),
        Vec::new(),
        Vec::new(),
        vec![def_id],
    ));
    assert_eq!(
        pick_variable_offset(&arena, var_id, &SourceIndex::build(raw)),
        Utf16CodeUnitOffset(11)
    );
}

#[test]
fn returns_zero_when_variable_has_no_identifiers_and_no_defs() {
    let raw = "// —\nconst x = 1;\n";
    let mut arena = empty_arena();
    let var_id = arena.variables.push(VariableData::new(
        "x".to_string(),
        placeholder_scope_id(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ));
    assert_eq!(
        pick_variable_offset(&arena, var_id, &SourceIndex::build(raw)),
        Utf16CodeUnitOffset(0)
    );
}
