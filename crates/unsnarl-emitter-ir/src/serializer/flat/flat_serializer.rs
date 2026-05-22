//! `FlatSerializer`: `IRSerializer` implementation that emits
//! string-id-based scopes, variables, and references.

use std::collections::HashMap;

use unsnarl_emitter::{IRSerializer, SerializeContext};
use unsnarl_ir::serialized::serialized_ir::SERIALIZED_IR_VERSION;
use unsnarl_ir::serialized::{
    SerializedIR, SerializedReferenceId, SerializedScopeId, SerializedSource, SerializedVariableId,
};
use unsnarl_ir::{ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::collect_scopes_in_order::collect_scopes_in_order;
use crate::serializer::flat::has_declaring_def::has_declaring_def;
use crate::serializer::flat::offset_of_identifier::offset_of_identifier;
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

        let scopes = {
            let _span = tracing::info_span!("flat::collect_scopes").entered();
            collect_scopes_in_order(arena, ctx.root_scope)
        };
        let scope_ids: HashMap<ScopeId, SerializedScopeId> = {
            let _span = tracing::info_span!("flat::scope_ids", count = scopes.len()).entered();
            let mut m = HashMap::with_capacity(scopes.len());
            for (i, &s) in scopes.iter().enumerate() {
                m.insert(s, SerializedScopeId::new(format!("scope#{i}")));
            }
            m
        };

        // Build variable_ids in scope iteration order, skipping
        // implicit-`arguments` bindings (`defs.len() == 0`) so the
        // serialized output never contains a zero-def variable. Track
        // the ordered list so we can later iterate it in the same
        // order we assigned IDs.
        let (variable_ids, ordered_variables): (
            HashMap<VariableId, SerializedVariableId>,
            Vec<VariableId>,
        ) = {
            let _span = tracing::info_span!("flat::variable_ids").entered();
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
                    let id =
                        SerializedVariableId::new(format!("{}:{}@{}", sid_str(sid), name, offset));
                    variable_ids.insert(v, id);
                    ordered_variables.push(v);
                }
            }
            tracing::info!(count = ordered_variables.len(), "variable ids built");
            (variable_ids, ordered_variables)
        };

        // References across all scopes, deduplicated by ID, sorted by
        // identifier offset.
        let all_references: Vec<ReferenceId> = {
            let _span = tracing::info_span!("flat::collect_references").entered();
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
            tracing::info!(count = all_references.len(), "references collected");
            all_references
        };
        let all_references = {
            let _span = tracing::info_span!("flat::sort_references", count = all_references.len())
                .entered();
            let mut all_references = all_references;
            all_references.sort_by_key(|&r| offset_of_identifier(&arena.references[r].identifier));
            all_references
        };
        let reference_ids: HashMap<ReferenceId, SerializedReferenceId> = {
            let _span =
                tracing::info_span!("flat::reference_ids", count = all_references.len()).entered();
            let mut m = HashMap::new();
            for (i, &r) in all_references.iter().enumerate() {
                m.insert(r, SerializedReferenceId::new(format!("ref#{i}")));
            }
            m
        };

        let serialized_scopes = {
            let _span =
                tracing::info_span!("flat::serialize_scopes", count = scopes.len()).entered();
            scopes
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
                .collect()
        };

        let serialized_variables = {
            let _span =
                tracing::info_span!("flat::serialize_variables", count = ordered_variables.len())
                    .entered();
            ordered_variables
                .iter()
                .map(|&v| {
                    serialize_variable(arena, v, &scope_ids, &variable_ids, &reference_ids, raw)
                })
                .collect()
        };

        let serialized_references = {
            let _span =
                tracing::info_span!("flat::serialize_references", count = all_references.len())
                    .entered();
            all_references
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
                .collect()
        };

        let unused_variable_ids: Vec<SerializedVariableId> = {
            let _span = tracing::info_span!("flat::unused").entered();
            let mut v = Vec::new();
            for &var in &ordered_variables {
                if annotations.of_variable(var).is_unused && has_declaring_def(arena, var) {
                    if let Some(id) = variable_ids.get(&var) {
                        v.push(id.clone());
                    }
                }
            }
            v
        };

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
