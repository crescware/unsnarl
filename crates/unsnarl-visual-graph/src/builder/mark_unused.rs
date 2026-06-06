//! Flag every IR-reported unused variable's node as unused on the
//! arena (skipping `var` declarations, which are excluded upstream).

use std::collections::HashSet;

use unsnarl_ir::serialized::SerializedIR;

use super::arena::BuildArena;
use super::node_id::node_id;

pub fn mark_unused(arena: &mut BuildArena, ir: &SerializedIR, var_var_ids: &HashSet<&str>) {
    for id in &ir.unused_variable_ids {
        if var_var_ids.contains(id.value()) {
            continue;
        }
        let target = node_id(id.value());
        for node in arena.nodes.iter_mut() {
            if node.id() == target {
                node.set_unused(true);
                break;
            }
        }
    }
}
