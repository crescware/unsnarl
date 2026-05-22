//! Walk variables and capture the binding ids that need wrapping
//! plus the `init` replacements for `useCallback`.
//!
//! For every variable whose first def is `Variable` with an `init`
//! of type `CallExpression`, it:
//! 1. Checks the call resolves to a tracked hook import via
//!    [`find_hook_callee_kind`].
//! 2. Locates the inner function scope via
//!    [`find_inner_function_scope`].
//! 3. Records the variable id in the `wrapped` set.
//! 4. For `useCallback`, additionally records an
//!    [`InitReplacement`] pointing at the inner block; for
//!    `useMemo`, the init stays as the original call so the IR
//!    reads as an IIFE-style invocation.

use std::collections::{HashMap, HashSet};

use unsnarl_ir::serialized::{SerializedDefinition, SerializedIR, SerializedScope};
use unsnarl_oxc_parity::AstType;

use super::find_hook_callee_kind::find_hook_callee_kind;
use super::find_inner_function_scope::find_inner_function_scope;
use super::hook_kind::HookKind;
use super::init_replacement::InitReplacement;

pub fn collect_init_targets(
    ir: &SerializedIR,
    hook_imports: &HashMap<String, HookKind>,
    child_scopes_by_upper: &HashMap<String, Vec<&SerializedScope>>,
) -> (HashMap<String, InitReplacement>, HashSet<String>) {
    let mut init_replacements: HashMap<String, InitReplacement> = HashMap::new();
    let mut wrapped_var_ids: HashSet<String> = HashSet::new();

    for v in &ir.variables {
        let Some(SerializedDefinition::Variable(vdef)) = v.defs.first() else {
            continue;
        };
        let Some(init) = vdef.init() else {
            continue;
        };
        if !matches!(init.r#type, AstType::CallExpression) {
            continue;
        }
        let owner_id = v.id.value();
        let Some(kind) = find_hook_callee_kind(ir, hook_imports, owner_id, &init.span) else {
            continue;
        };
        let empty: Vec<&SerializedScope> = Vec::new();
        let siblings = child_scopes_by_upper.get(v.scope.value()).unwrap_or(&empty);
        let Some(inner) = find_inner_function_scope(siblings, init.span.offset) else {
            continue;
        };
        wrapped_var_ids.insert(owner_id.to_string());
        if matches!(kind, HookKind::UseCallback) {
            init_replacements.insert(
                owner_id.to_string(),
                InitReplacement {
                    ty: inner.block.r#type.clone(),
                    span: inner.block.span.clone(),
                },
            );
        }
    }

    (init_replacements, wrapped_var_ids)
}
