//! Collect every variable that imports a tracked React hook.
//!
//! Walks every `SerializedVariable`, keeps the ones whose first
//! definition is a `ImportBinding` (named) sourced from `"react"`
//! with an `importedName` matching one of [`HOOK_USE_CALLBACK`] /
//! [`HOOK_USE_MEMO`], and yields a map keyed by the variable id's
//! string value.

use std::collections::HashMap;

use unsnarl_ir::serialized::{SerializedDefinition, SerializedIR};

use super::hook_kind::{as_hook_kind, HookKind, REACT_MODULE};

pub fn collect_hook_imports(ir: &SerializedIR) -> HashMap<String, HookKind> {
    let mut out = HashMap::new();
    for v in &ir.variables {
        let Some(def) = v.defs.first() else {
            continue;
        };
        let SerializedDefinition::ImportBindingNamed(named) = def else {
            continue;
        };
        if named.import_source() != REACT_MODULE {
            continue;
        }
        let Some(kind) = as_hook_kind(named.imported_name()) else {
            continue;
        };
        out.insert(v.id.value().to_string(), kind);
    }
    out
}
