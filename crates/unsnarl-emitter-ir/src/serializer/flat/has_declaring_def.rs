//! Does the variable have at least one non-`ImplicitGlobalVariable`
//! definition?
//!
//! Mirrors `hasDeclaringDef` in
//! `ts/src/serializer/flat/has-declaring-def.ts`.

use unsnarl_ir::{DefinitionType, IrArena, VariableId};

pub fn has_declaring_def(arena: &IrArena, variable: VariableId) -> bool {
    arena.variables[variable]
        .defs
        .iter()
        .any(|&id| arena.definitions[id].r#type != DefinitionType::ImplicitGlobalVariable)
}
