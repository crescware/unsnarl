//! `FlatSerializer`: `IRSerializer` implementation that emits
//! string-id-based scopes, variables, and references.

use std::collections::HashMap;
use std::time::Duration;

use unsnarl_emitter::{IRSerializer, SerializeContext};
use unsnarl_instrumentation::{add_elapsed, timing_start, verbose};
use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::serialized::serialized_ir::SERIALIZED_IR_VERSION;
use unsnarl_ir::serialized::{
    SerializedIR, SerializedReferenceId, SerializedScopeId, SerializedSource, SerializedVariableId,
};
use unsnarl_ir::{ReferenceId, ScopeId, VariableId};

use crate::serializer::flat::collect_scopes_in_order::collect_scopes_in_order;
use crate::serializer::flat::has_declaring_def::has_declaring_def;
use crate::serializer::flat::offset_of_identifier::offset_of_identifier;
use crate::serializer::flat::pick_variable_offset::pick_variable_offset;
use crate::serializer::flat::serialize_reference::{
    self as serialize_reference_mod, serialize_reference,
};
use crate::serializer::flat::serialize_scope::{self as serialize_scope_mod, serialize_scope};
use crate::serializer::flat::serialize_variable::{
    self as serialize_variable_mod, serialize_variable,
};

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

        let index = {
            let _span =
                unsnarl_instrumentation::span!("flat::build_source_index", bytes = raw.len());
            SourceIndex::build(raw)
        };

        let scopes = {
            let _span = unsnarl_instrumentation::span!("flat::collect_scopes");
            collect_scopes_in_order(arena, ctx.root_scope)
        };
        let scope_ids: HashMap<ScopeId, SerializedScopeId> = {
            let _span = unsnarl_instrumentation::span!("flat::scope_ids", count = scopes.len());
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
            let _span = unsnarl_instrumentation::span!("flat::variable_ids");
            let mut t_arena_lookup = Duration::ZERO;
            let mut t_scope_lookup = Duration::ZERO;
            let mut t_pick_offset = Duration::ZERO;
            let mut t_format_id = Duration::ZERO;
            let mut t_insert = Duration::ZERO;
            let mut variable_ids: HashMap<VariableId, SerializedVariableId> = HashMap::new();
            let mut ordered_variables: Vec<VariableId> = Vec::new();
            for &s in &scopes {
                for &v in &arena.scopes[s].variables {
                    let t = timing_start();
                    let is_empty = arena.variables[v].defs.is_empty();
                    add_elapsed(&mut t_arena_lookup, t);
                    if is_empty {
                        continue;
                    }
                    let t = timing_start();
                    let sid_opt = scope_ids.get(&s);
                    add_elapsed(&mut t_scope_lookup, t);
                    let Some(sid) = sid_opt else {
                        continue;
                    };
                    let t = timing_start();
                    let offset = pick_variable_offset(arena, v, &index);
                    add_elapsed(&mut t_pick_offset, t);
                    let t = timing_start();
                    let name = arena.variables[v].name();
                    let id = SerializedVariableId::new(format!(
                        "{}:{}@{}",
                        sid_str(sid),
                        name,
                        offset.0
                    ));
                    add_elapsed(&mut t_format_id, t);
                    let t = timing_start();
                    variable_ids.insert(v, id);
                    ordered_variables.push(v);
                    add_elapsed(&mut t_insert, t);
                }
            }
            if verbose() {
                tracing::info!(
                    count = ordered_variables.len(),
                    arena_lookup_ms = t_arena_lookup.as_millis() as u64,
                    scope_lookup_ms = t_scope_lookup.as_millis() as u64,
                    pick_offset_ms = t_pick_offset.as_millis() as u64,
                    format_id_ms = t_format_id.as_millis() as u64,
                    insert_ms = t_insert.as_millis() as u64,
                    "variable ids built",
                );
            }
            (variable_ids, ordered_variables)
        };

        // References across all scopes, deduplicated by ID, sorted by
        // identifier offset.
        let all_references: Vec<ReferenceId> = {
            let _span = unsnarl_instrumentation::span!("flat::collect_references");
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
            if verbose() {
                tracing::info!(count = all_references.len(), "references collected");
            }
            all_references
        };
        let all_references = {
            let _span = unsnarl_instrumentation::span!(
                "flat::sort_references",
                count = all_references.len()
            );
            let mut all_references = all_references;
            all_references.sort_by_key(|&r| offset_of_identifier(&arena.references[r].identifier));
            all_references
        };
        let reference_ids: HashMap<ReferenceId, SerializedReferenceId> = {
            let _span =
                unsnarl_instrumentation::span!("flat::reference_ids", count = all_references.len());
            let mut m = HashMap::new();
            for (i, &r) in all_references.iter().enumerate() {
                m.insert(r, SerializedReferenceId::new(format!("ref#{i}")));
            }
            m
        };

        let serialized_scopes = {
            let _span =
                unsnarl_instrumentation::span!("flat::serialize_scopes", count = scopes.len());
            serialize_scope_mod::reset_stats();
            let out: Vec<_> = scopes
                .iter()
                .map(|&s| {
                    serialize_scope(
                        arena,
                        s,
                        &scope_ids,
                        &variable_ids,
                        &reference_ids,
                        annotations,
                        &index,
                    )
                })
                .collect();
            serialize_scope_mod::emit_stats();
            out
        };

        let serialized_variables = {
            let _span = unsnarl_instrumentation::span!(
                "flat::serialize_variables",
                count = ordered_variables.len()
            );
            serialize_variable_mod::reset_stats();
            let out: Vec<_> = ordered_variables
                .iter()
                .map(|&v| {
                    serialize_variable(arena, v, &scope_ids, &variable_ids, &reference_ids, &index)
                })
                .collect();
            serialize_variable_mod::emit_stats();
            out
        };

        let serialized_references = {
            let _span = unsnarl_instrumentation::span!(
                "flat::serialize_references",
                count = all_references.len()
            );
            // Reset the per-sub-phase accumulators so the summary
            // emitted after this loop reflects only this loop's calls.
            serialize_reference_mod::reset_stats();
            let out: Vec<_> = all_references
                .iter()
                .map(|&r| {
                    serialize_reference(
                        arena,
                        r,
                        &scope_ids,
                        &variable_ids,
                        &reference_ids,
                        annotations,
                        &index,
                    )
                })
                .collect();
            serialize_reference_mod::emit_stats();
            out
        };

        let unused_variable_ids: Vec<SerializedVariableId> = {
            let _span = unsnarl_instrumentation::span!("flat::unused");
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
