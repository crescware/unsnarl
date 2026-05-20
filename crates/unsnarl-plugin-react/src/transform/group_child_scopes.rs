//! Group scopes by their parent (`upper`).
//!
//! Mirrors `groupChildScopes` in
//! `ts/src/plugins/unsnarl-plugin-react/index.ts`. Used by
//! `collectInitTargets` to look up sibling scopes of a wrapped
//! variable's owning scope when searching for the inner
//! `useFoo(() => ..., deps)` callback.

use std::collections::HashMap;

use unsnarl_ir::serialized::{SerializedIR, SerializedScope};

pub fn group_child_scopes(ir: &SerializedIR) -> HashMap<String, Vec<&SerializedScope>> {
    let mut out: HashMap<String, Vec<&SerializedScope>> = HashMap::new();
    for s in &ir.scopes {
        let Some(upper) = s.upper.as_ref() else {
            continue;
        };
        out.entry(upper.value().to_string()).or_default().push(s);
    }
    out
}
