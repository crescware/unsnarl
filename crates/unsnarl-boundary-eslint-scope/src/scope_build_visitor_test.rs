//! Sibling tests for `scope_build_visitor.rs`.
//!
//! Collapses the TS test surface for `handle-enter.test.ts`,
//! `handle-leave.test.ts`, `walk/walk.test.ts`, and
//! `eslint-compat.test.ts` because the Rust walker subsumes all four
//! into one `ScopeBuildVisitor` (each TS module's `case` arm is now
//! a `visit_*` override on this struct).

use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::Language;

use crate::testing::analyze_source;

#[test]
fn walker_descends_through_nested_blocks_and_pops_correctly() {
    // `{{{}}}` produces three nested Block scopes; the walker must
    // both push and pop each one. If pop_scope is off, the
    // `current_scope` panics on the next push.
    let r = analyze_source("{ { { let inner = 1; } } }\n", Language::Ts);
    let mut depth = 0;
    let mut cur = r.arena.scopes[r.global_scope].child_scopes.first().copied();
    while let Some(s) = cur {
        if !matches!(r.arena.scopes[s].r#type, ScopeType::Block) {
            break;
        }
        depth += 1;
        cur = r.arena.scopes[s].child_scopes.first().copied();
    }
    assert_eq!(depth, 3);
}

#[test]
fn walker_visits_every_scope_only_once() {
    // A nontrivial source must not double-create any scope; child
    // counts at each level are the obvious sanity check.
    let r = analyze_source("function outer() { function inner() {} }\n", Language::Ts);
    let outer = r.arena.scopes[r.global_scope].child_scopes.clone();
    assert_eq!(
        outer.len(),
        1,
        "exactly one direct child for `function outer`"
    );
    let outer_scope = outer[0];
    let inner = r.arena.scopes[outer_scope].child_scopes.clone();
    assert_eq!(inner.len(), 1, "exactly one inner function scope");
}

#[test]
fn eslint_compat_module_scope_chain_terminates_at_module_root() {
    let r = analyze_source("export const x = 1;\n", Language::Ts);
    // Module root has no upper.
    assert!(r.arena.scopes[r.global_scope].upper.is_none());
}
