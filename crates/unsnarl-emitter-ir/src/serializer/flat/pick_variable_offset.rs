//! Pick the offset used to spell a variable's serialized ID.
//!
//! Mirrors `pickVariableOffset` in
//! `ts/src/serializer/flat/pick-variable-offset.ts`. The TS source
//! falls back to `def.name.start ?? 0` if both the identifier head
//! and the first definition's name lack a start position; in the
//! Rust IR every identifier carries a span, so we use the head
//! identifier's span, then the first def's name span, then `0` as a
//! final fallback when the variable has no identifiers AND no defs.
//!
//! The npm `oxc-parser` package emits offsets in UTF-16 code units
//! (JavaScript string indices) so TS `head.start` is already in that
//! unit. The Rust `oxc_parser` crate emits offsets in UTF-8 bytes;
//! this function converts that byte offset to UTF-16 code units via
//! `span_from_offset` so the serialized variable ID
//! (`scope#N:name@offset`) matches the TS implementation byte-for-byte
//! in sources containing non-ASCII characters (e.g. an arrow `→` or
//! em-dash `—` in a docstring before the declaration).

use unsnarl_ir::primitive::span_from_offset;
use unsnarl_ir::{IrArena, VariableId};

pub fn pick_variable_offset(arena: &IrArena, variable: VariableId, raw: &str) -> u32 {
    let v = &arena.variables[variable];
    let byte_offset = if let Some(head) = v.identifiers.first() {
        head.span.start
    } else if let Some(&def_id) = v.defs.first() {
        arena.definitions[def_id].name.span.start
    } else {
        return 0;
    };
    span_from_offset(raw, byte_offset as usize).offset.0
}

#[cfg(test)]
#[path = "pick_variable_offset_test.rs"]
mod pick_variable_offset_test;
