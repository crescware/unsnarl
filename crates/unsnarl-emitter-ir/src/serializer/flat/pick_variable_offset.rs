//! Pick the offset used to spell a variable's serialized ID.
//!
//! Falls back through the head identifier's span, then the first
//! def's name span, then `0` as a final fallback when the variable
//! has no identifiers AND no defs.
//!
//! The serialized variable ID (`scope#N:name@offset`) records the
//! offset in UTF-16 code units. The `oxc_parser` crate emits offsets
//! in UTF-8 bytes; this function converts that byte offset via the
//! pre-computed `SourceIndex` so the encoded ID stays consistent in
//! sources containing non-ASCII characters (e.g. an arrow `→` or
//! em-dash `—` in a docstring before the declaration).

use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::{IrArena, VariableId};

pub fn pick_variable_offset(arena: &IrArena, variable: VariableId, index: &SourceIndex<'_>) -> u32 {
    let v = &arena.variables[variable];
    let byte_offset = if let Some(head) = v.identifiers.first() {
        head.span.start
    } else if let Some(&def_id) = v.defs.first() {
        arena.definitions[def_id].name.span.start
    } else {
        return 0;
    };
    index.span_at(byte_offset as usize).offset.0
}

#[cfg(test)]
#[path = "pick_variable_offset_test.rs"]
mod pick_variable_offset_test;
