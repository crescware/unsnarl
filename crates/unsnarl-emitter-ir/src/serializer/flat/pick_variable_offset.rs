//! Pick the offset used to spell a variable's serialized ID.
//!
//! Mirrors `pickVariableOffset` in
//! `ts/src/serializer/flat/pick-variable-offset.ts`. The TS source
//! falls back to `def.name.start ?? 0` if both the identifier head
//! and the first definition's name lack a start position; in the
//! Rust IR every identifier carries a span, so we use the head
//! identifier's span, then the first def's name span, then `0` as a
//! final fallback when the variable has no identifiers AND no defs.

use unsnarl_ir::{IrArena, VariableId};

pub fn pick_variable_offset(arena: &IrArena, variable: VariableId) -> u32 {
    let v = &arena.variables[variable];
    if let Some(head) = v.identifiers.first() {
        return head.span.start;
    }
    if let Some(&def_id) = v.defs.first() {
        return arena.definitions[def_id].name.span.start;
    }
    0
}
