//! `FlatSerializer`: TS-compatible `IRSerializer` implementation that
//! emits string-id-based scopes, variables, and references.
//!
//! Mirrors `FlatSerializer` in
//! `ts/src/serializer/flat/flat-serializer.ts`.

use std::collections::HashMap;

use unsnarl_emitter::{IRSerializer, SerializeContext};
use unsnarl_ir::serialized::serialized_ir::SERIALIZED_IR_VERSION;
use unsnarl_ir::serialized::{
    SerializedIR, SerializedReferenceId, SerializedScopeId, SerializedSource, SerializedVariableId,
};
use unsnarl_ir::{ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::collect_scopes_in_order::collect_scopes_in_order;
use crate::serializer::flat::has_declaring_def::has_declaring_def;
use crate::serializer::flat::offset_of::offset_of_identifier;
use crate::serializer::flat::pick_variable_offset::pick_variable_offset;
use crate::serializer::flat::serialize_reference::serialize_reference;
use crate::serializer::flat::serialize_scope::serialize_scope;
use crate::serializer::flat::serialize_variable::serialize_variable;

pub struct FlatSerializer;

impl FlatSerializer {
    pub const ID: &'static str = "flat";
}

impl Default for FlatSerializer {
    fn default() -> Self {
        Self
    }
}

impl IRSerializer for FlatSerializer {
    fn id(&self) -> &'static str {
        Self::ID
    }

    fn serialize(&self, ctx: &SerializeContext<'_>) -> SerializedIR {
        let arena = ctx.arena;
        let raw = ctx.raw;
        let annotations = ctx.annotations;

        let scopes = collect_scopes_in_order(arena, ctx.root_scope);
        let mut scope_ids: HashMap<ScopeId, SerializedScopeId> =
            HashMap::with_capacity(scopes.len());
        for (i, &s) in scopes.iter().enumerate() {
            scope_ids.insert(s, SerializedScopeId::new(format!("scope#{i}")));
        }

        // Build variable_ids in scope iteration order, skipping
        // implicit-`arguments` bindings (`defs.len() == 0`) so the
        // serialized output never contains a zero-def variable. Track
        // the ordered list so we can later iterate it in the same
        // order we assigned IDs.
        let mut variable_ids: HashMap<VariableId, SerializedVariableId> = HashMap::new();
        let mut ordered_variables: Vec<VariableId> = Vec::new();
        for &s in &scopes {
            for &v in &arena.scopes[s].variables {
                if arena.variables[v].defs.is_empty() {
                    continue;
                }
                let Some(sid) = scope_ids.get(&s) else {
                    continue;
                };
                let offset = pick_variable_offset(arena, v, raw);
                let name = arena.variables[v].name();
                let id = SerializedVariableId::new(format!("{}:{}@{}", sid_str(sid), name, offset));
                variable_ids.insert(v, id);
                ordered_variables.push(v);
            }
        }

        // References across all scopes, deduplicated by ID, sorted by
        // identifier offset.
        let mut all_references: Vec<ReferenceId> = Vec::new();
        let mut seen_references: std::collections::HashSet<ReferenceId> =
            std::collections::HashSet::new();
        for &s in &scopes {
            for &r in &arena.scopes[s].references {
                if seen_references.insert(r) {
                    all_references.push(r);
                }
            }
        }
        all_references.sort_by_key(|&r| offset_of_identifier(&arena.references[r].identifier));
        let mut reference_ids: HashMap<ReferenceId, SerializedReferenceId> = HashMap::new();
        for (i, &r) in all_references.iter().enumerate() {
            reference_ids.insert(r, SerializedReferenceId::new(format!("ref#{i}")));
        }

        let serialized_scopes = scopes
            .iter()
            .map(|&s| {
                serialize_scope(
                    arena,
                    s,
                    &scope_ids,
                    &variable_ids,
                    &reference_ids,
                    annotations,
                    raw,
                )
            })
            .collect();

        let serialized_variables = ordered_variables
            .iter()
            .map(|&v| serialize_variable(arena, v, &scope_ids, &variable_ids, &reference_ids, raw))
            .collect();

        let serialized_references = all_references
            .iter()
            .map(|&r| {
                serialize_reference(
                    arena,
                    r,
                    &scope_ids,
                    &variable_ids,
                    &reference_ids,
                    annotations,
                    raw,
                )
            })
            .collect();

        let mut unused_variable_ids: Vec<SerializedVariableId> = Vec::new();
        for &v in &ordered_variables {
            if annotations.of_variable(v).is_unused && has_declaring_def(arena, v) {
                if let Some(id) = variable_ids.get(&v) {
                    unused_variable_ids.push(id.clone());
                }
            }
        }

        SerializedIR {
            version: SERIALIZED_IR_VERSION,
            source: SerializedSource {
                path: ctx.source.path.clone(),
                language: ctx.source.language,
            },
            raw: raw.to_string(),
            scopes: serialized_scopes,
            variables: serialized_variables,
            references: serialized_references,
            unused_variable_ids,
            diagnostics: ctx.diagnostics.to_vec(),
        }
    }
}

fn sid_str(id: &SerializedScopeId) -> &str {
    id.value()
}
