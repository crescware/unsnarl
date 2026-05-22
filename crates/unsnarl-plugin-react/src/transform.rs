//! IR transform: peel `useCallback` / `useMemo` so the wrapped
//! callback's body shows up as a normal function in the IR.
//!
//! The helpers (`collect_hook_imports`, `group_child_scopes`,
//! `collect_init_targets`, ...) each carry their own per-step
//! commentary.
//!
//! The transform takes ownership of the [`SerializedIR`] and
//! returns the rewritten value. Steps:
//!
//! 1. Collect `react` hook imports (`useCallback` / `useMemo`).
//! 2. Index every scope by its parent (`upper`).
//! 3. For each `const x = useFoo(() => ..., deps)` binding, locate
//!    the inner function scope and record the wrapper variable id.
//!    For `useCallback`, also remember the inner block so the
//!    binding's `init` points at the function instead of the call.
//! 4. Drop all non-init references owned by the wrapped variables
//!    (the `() => ...` body / dependency array references).
//! 5. Drop hook imports whose only remaining references were
//!    inside the dropped set.
//! 6. Rewrite the IR: filter scope / variable / reference lists,
//!    apply the saved `init` replacements, and re-emit.

mod collect_hook_imports;
mod collect_init_targets;
mod collect_refs_to_remove;
mod collect_vars_to_remove;
mod count_retained_refs_by_resolved;
mod find_hook_callee_kind;
mod find_inner_function_scope;
mod group_child_scopes;
mod hook_kind;
mod init_replacement;
mod rebuild_ir;

use unsnarl_ir::serialized::SerializedIR;

use collect_hook_imports::collect_hook_imports;
use collect_init_targets::collect_init_targets;
use collect_refs_to_remove::collect_refs_to_remove;
use collect_vars_to_remove::collect_vars_to_remove;
use count_retained_refs_by_resolved::count_retained_refs_by_resolved;
use group_child_scopes::group_child_scopes;
use rebuild_ir::{rebuild_ir, IrChanges};

pub fn transform_ir(ir: SerializedIR) -> SerializedIR {
    let hook_imports = collect_hook_imports(&ir);
    if hook_imports.is_empty() {
        return ir;
    }
    let child_scopes_by_upper = group_child_scopes(&ir);
    let (init_replacements, wrapped_var_ids) =
        collect_init_targets(&ir, &hook_imports, &child_scopes_by_upper);
    if wrapped_var_ids.is_empty() {
        return ir;
    }
    let refs_to_remove = collect_refs_to_remove(&ir, &wrapped_var_ids);
    let refs_retained_by_var = count_retained_refs_by_resolved(&ir, &refs_to_remove);
    let vars_to_remove = collect_vars_to_remove(&hook_imports, &refs_retained_by_var);
    rebuild_ir(
        ir,
        &IrChanges {
            refs_to_remove,
            vars_to_remove,
            init_replacements,
        },
    )
}
