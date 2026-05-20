//! Resolve a wrapped binding's `init` call to a hook import.
//!
//! Mirrors `findHookCalleeKind` in
//! `ts/src/plugins/unsnarl-plugin-react/index.ts`. Walks every
//! reference and keeps the first one that:
//! - is not the binding's own `init` reference (`r.init === false`),
//! - is a call site (`flags.call`),
//! - resolves to one of the tracked hook imports,
//! - lists `ownerVarId` among its `owners`,
//! - whose identifier span offset equals the binding's `init` span
//!   offset (the callee identifier sits exactly at the start of the
//!   `CallExpression`).

use std::collections::HashMap;

use unsnarl_ir::primitive::Span;
use unsnarl_ir::serialized::SerializedIR;

use super::hook_kind::HookKind;

pub fn find_hook_callee_kind(
    ir: &SerializedIR,
    hook_imports: &HashMap<String, HookKind>,
    owner_var_id: &str,
    init_span: &Span,
) -> Option<HookKind> {
    for r in &ir.references {
        if r.init {
            continue;
        }
        if !r.flags.call {
            continue;
        }
        let Some(resolved) = r.resolved.as_ref() else {
            continue;
        };
        let Some(kind) = hook_imports.get(resolved.value()) else {
            continue;
        };
        if !r.owners.iter().any(|o| o.value() == owner_var_id) {
            continue;
        }
        if r.identifier.span().offset != init_span.offset {
            continue;
        }
        return Some(*kind);
    }
    None
}
