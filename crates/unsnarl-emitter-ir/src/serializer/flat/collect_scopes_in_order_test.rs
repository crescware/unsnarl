//! Mirrors `ts/src/serializer/flat/collect-scopes-in-order.test.ts`.
//! Pins the depth-first pre-order traversal of `child_scopes`
//! starting from the root.

use oxc_index::IndexVec;
use oxc_span::Span;

use unsnarl_ir::primitive::AstNode;
use unsnarl_ir::scope::ScopeData;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::{IrArena, ScopeId};
use unsnarl_oxc_parity::AstType;

use super::collect_scopes_in_order;

fn empty_arena() -> IrArena {
    IrArena {
        scopes: IndexVec::new(),
        variables: IndexVec::new(),
        references: IndexVec::new(),
        definitions: IndexVec::new(),
    }
}

fn push_scope(arena: &mut IrArena, r#type: ScopeType, children: Vec<ScopeId>) -> ScopeId {
    // `variable_scope` is irrelevant to `collect_scopes_in_order`
    // (it only walks `child_scopes`); use a placeholder so the test
    // can build the tree without resolving the variable-scope graph.
    let placeholder = ScopeId::from_usize(0);
    arena.scopes.push(ScopeData::new(
        r#type,
        true,
        None,
        children,
        placeholder,
        AstNode {
            r#type: AstType::Program,
            span: Span::new(0, 0),
        },
        Vec::new(),
        Vec::new(),
        Vec::new(),
        false,
    ))
}

#[test]
fn returns_the_root_alone_when_there_are_no_children() {
    let mut arena = empty_arena();
    let root = push_scope(&mut arena, ScopeType::Module, Vec::new());
    assert_eq!(collect_scopes_in_order(&arena, root), vec![root]);
}

#[test]
fn performs_a_depth_first_pre_order_traversal_of_child_scopes() {
    let mut arena = empty_arena();
    // Push leaves first so the parents' `child_scopes` can reference
    // them. The traversal order is determined by the parent's
    // `child_scopes` Vec ordering, not by insertion order in the
    // arena.
    let a1 = push_scope(&mut arena, ScopeType::Block, Vec::new());
    let b = push_scope(&mut arena, ScopeType::Block, Vec::new());
    let a = push_scope(&mut arena, ScopeType::Block, vec![a1]);
    let root = push_scope(&mut arena, ScopeType::Module, vec![a, b]);

    assert_eq!(collect_scopes_in_order(&arena, root), vec![root, a, a1, b]);
}
