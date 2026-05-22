//! Pre-order DFS over the scope tree starting at `root`.

use unsnarl_ir::{IrArena, ScopeId};

pub fn collect_scopes_in_order(arena: &IrArena, root: ScopeId) -> Vec<ScopeId> {
    let mut out = Vec::new();
    visit(arena, root, &mut out);
    out
}

fn visit(arena: &IrArena, scope: ScopeId, out: &mut Vec<ScopeId>) {
    out.push(scope);
    for &child in &arena.scopes[scope].child_scopes {
        visit(arena, child, out);
    }
}

#[cfg(test)]
#[path = "collect_scopes_in_order_test.rs"]
mod collect_scopes_in_order_test;
