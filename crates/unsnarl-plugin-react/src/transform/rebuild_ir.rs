//! Apply the collected deletions / replacements to the IR.
//!
//! Mutates the owned IR in place since the transform takes
//! ownership of the value:
//!
//! - `ir.references` is filtered against `refs_to_remove`.
//! - `ir.variables` is filtered against `vars_to_remove`. For each
//!   retained variable, its per-variable `references` list is also
//!   filtered, and — when the binding is a wrapped `useCallback` —
//!   its `defs[0].init` is replaced with a [`DefinitionNode`]
//!   pointing at the inner function block.
//! - Each scope's `variables` / `references` / `through` lists are
//!   filtered against the deletions.
//! - `ir.unused_variable_ids` is filtered against `vars_to_remove`.

use std::collections::{HashMap, HashSet};

use unsnarl_ir::serialized::{DefinitionNode, SerializedDefinition, SerializedIR};

use super::init_replacement::InitReplacement;

pub struct IrChanges {
    pub refs_to_remove: HashSet<String>,
    pub vars_to_remove: HashSet<String>,
    pub init_replacements: HashMap<String, InitReplacement>,
}

pub fn rebuild_ir(mut ir: SerializedIR, changes: &IrChanges) -> SerializedIR {
    ir.references
        .retain(|r| !changes.refs_to_remove.contains(r.id.value()));

    ir.variables
        .retain(|v| !changes.vars_to_remove.contains(v.id.value()));
    for v in &mut ir.variables {
        v.references
            .retain(|rid| !changes.refs_to_remove.contains(rid.value()));
        if let Some(repl) = changes.init_replacements.get(v.id.value()) {
            if let Some(SerializedDefinition::Variable(vdef)) = v.defs.first_mut() {
                vdef.set_init(Some(DefinitionNode {
                    r#type: repl.ty.clone(),
                    span: repl.span.clone(),
                }));
            }
        }
    }

    for s in &mut ir.scopes {
        s.variables
            .retain(|vid| !changes.vars_to_remove.contains(vid.value()));
        s.references
            .retain(|rid| !changes.refs_to_remove.contains(rid.value()));
        s.through
            .retain(|rid| !changes.refs_to_remove.contains(rid.value()));
    }

    ir.unused_variable_ids
        .retain(|vid| !changes.vars_to_remove.contains(vid.value()));

    ir
}
