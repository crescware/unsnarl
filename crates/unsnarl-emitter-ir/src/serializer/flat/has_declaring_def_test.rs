//! Mirrors `ts/src/serializer/flat/has-declaring-def.test.ts`. Pins
//! the rule "at least one def whose type is NOT
//! `ImplicitGlobalVariable`" — anything else (including the empty
//! defs list) is treated as not having a declaring def.

use oxc_index::IndexVec;
use oxc_span::Span;

use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::{DefinitionData, VariableData};
use unsnarl_ir::{DefinitionType, IrArena, ScopeId};
use unsnarl_oxc_parity::AstType;

use super::has_declaring_def;

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

fn push_def(arena: &mut IrArena, ty: DefinitionType) -> unsnarl_ir::DefinitionId {
    arena.definitions.push(DefinitionData {
        r#type: ty,
        name: AstIdentifier::new(AstType::Identifier, "x".to_string(), Span::new(0, 1)),
        node: AstNode {
            r#type: AstType::Identifier,
            span: Span::new(0, 1),
        },
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    })
}

fn push_var(arena: &mut IrArena, defs: Vec<unsnarl_ir::DefinitionId>) -> unsnarl_ir::VariableId {
    arena.variables.push(VariableData::new(
        "x".to_string(),
        placeholder_scope_id(),
        Vec::new(),
        Vec::new(),
        defs,
    ))
}

#[test]
fn true_when_at_least_one_def_has_a_non_implicit_type() {
    let mut arena = empty_arena();
    let def = push_def(&mut arena, DefinitionType::Variable);
    let v = push_var(&mut arena, vec![def]);
    assert!(has_declaring_def(&arena, v));
}

#[test]
fn false_when_every_def_is_implicit_global_variable() {
    let mut arena = empty_arena();
    let def = push_def(&mut arena, DefinitionType::ImplicitGlobalVariable);
    let v = push_var(&mut arena, vec![def]);
    assert!(!has_declaring_def(&arena, v));
}

#[test]
fn true_when_mixed_any_single_non_implicit_def_is_enough() {
    let mut arena = empty_arena();
    let implicit = push_def(&mut arena, DefinitionType::ImplicitGlobalVariable);
    let func = push_def(&mut arena, DefinitionType::FunctionName);
    let v = push_var(&mut arena, vec![implicit, func]);
    assert!(has_declaring_def(&arena, v));
}

#[test]
fn false_when_defs_is_empty() {
    let mut arena = empty_arena();
    let v = push_var(&mut arena, Vec::new());
    assert!(!has_declaring_def(&arena, v));
}
