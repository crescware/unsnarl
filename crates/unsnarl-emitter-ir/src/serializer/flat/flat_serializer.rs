//! `FlatSerializer`: `IRSerializer` implementation that emits
//! string-id-based scopes, variables, and references.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use unsnarl_emitter::{IRSerializer, SerializeContext};
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
use crate::serializer::flat::serialize_expression_statement_head::take_head_stats;
use crate::serializer::flat::serialize_reference::{
    serialize_reference, take_serialize_reference_stats,
};
use crate::serializer::flat::serialize_scope::{serialize_scope, take_serialize_scope_stats};
use crate::serializer::flat::serialize_variable::{
    serialize_variable, take_serialize_variable_stats,
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
                tracing::info_span!("flat::build_source_index", bytes = raw.len()).entered();
            SourceIndex::build(raw)
        };

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
            let mut t_arena_lookup = Duration::ZERO;
            let mut t_scope_lookup = Duration::ZERO;
            let mut t_pick_offset = Duration::ZERO;
            let mut t_format_id = Duration::ZERO;
            let mut t_insert = Duration::ZERO;
            let mut variable_ids: HashMap<VariableId, SerializedVariableId> = HashMap::new();
            let mut ordered_variables: Vec<VariableId> = Vec::new();
            for &s in &scopes {
                for &v in &arena.scopes[s].variables {
                    let t = Instant::now();
                    let is_empty = arena.variables[v].defs.is_empty();
                    t_arena_lookup += t.elapsed();
                    if is_empty {
                        continue;
                    }
                    let t = Instant::now();
                    let sid_opt = scope_ids.get(&s);
                    t_scope_lookup += t.elapsed();
                    let Some(sid) = sid_opt else {
                        continue;
                    };
                    let t = Instant::now();
                    let offset = pick_variable_offset(arena, v, &index);
                    t_pick_offset += t.elapsed();
                    let t = Instant::now();
                    let name = arena.variables[v].name();
                    let id =
                        SerializedVariableId::new(format!("{}:{}@{}", sid_str(sid), name, offset));
                    t_format_id += t.elapsed();
                    let t = Instant::now();
                    variable_ids.insert(v, id);
                    ordered_variables.push(v);
                    t_insert += t.elapsed();
                }
            }
            tracing::info!(
                count = ordered_variables.len(),
                arena_lookup_ms = t_arena_lookup.as_millis() as u64,
                scope_lookup_ms = t_scope_lookup.as_millis() as u64,
                pick_offset_ms = t_pick_offset.as_millis() as u64,
                format_id_ms = t_format_id.as_millis() as u64,
                insert_ms = t_insert.as_millis() as u64,
                "variable ids built",
            );
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
            let _ = take_serialize_scope_stats();
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
            let st = take_serialize_scope_stats();
            tracing::info!(
                lookup_ms = st.lookup_ns / 1_000_000,
                child_scopes_ms = st.child_scopes_ns / 1_000_000,
                block_ms = st.block_ns / 1_000_000,
                variables_ms = st.variables_ns / 1_000_000,
                references_ms = st.references_ns / 1_000_000,
                through_ms = st.through_ns / 1_000_000,
                annotations_ms = st.annotations_ns / 1_000_000,
                build_ms = st.build_ns / 1_000_000,
                child_scopes_total = st.child_scopes_total,
                variables_total = st.variables_total,
                references_total = st.references_total,
                through_total = st.through_total,
                "serialize_scope sub-phase totals",
            );
            out
        };

        let serialized_variables = {
            let _span =
                tracing::info_span!("flat::serialize_variables", count = ordered_variables.len())
                    .entered();
            let _ = take_serialize_variable_stats();
            let out: Vec<_> = ordered_variables
                .iter()
                .map(|&v| {
                    serialize_variable(arena, v, &scope_ids, &variable_ids, &reference_ids, &index)
                })
                .collect();
            let st = take_serialize_variable_stats();
            tracing::info!(
                lookup_ms = st.lookup_ns / 1_000_000,
                identifiers_ms = st.identifiers_ns / 1_000_000,
                references_ms = st.references_ns / 1_000_000,
                defs_ms = st.defs_ns / 1_000_000,
                build_ms = st.build_ns / 1_000_000,
                identifiers_total = st.identifiers_total,
                references_total = st.references_total,
                defs_total = st.defs_total,
                "serialize_variable sub-phase totals",
            );
            out
        };

        let serialized_references = {
            let _span =
                tracing::info_span!("flat::serialize_references", count = all_references.len())
                    .entered();
            // Reset the per-sub-phase accumulators so the summary
            // emitted after this loop reflects only this loop's
            // calls.
            let _ = take_serialize_reference_stats();
            let _ = take_head_stats();
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
            let stats = take_serialize_reference_stats();
            let head = take_head_stats();
            tracing::info!(
                lookup_ms = stats.lookup_ns / 1_000_000,
                annotations_ms = stats.annotations_ns / 1_000_000,
                owners_ms = stats.owners_ns / 1_000_000,
                completion_ms = stats.completion_ns / 1_000_000,
                jsx_ms = stats.jsx_ns / 1_000_000,
                expr_stmt_container_ms = stats.expression_statement_container_ns / 1_000_000,
                expr_stmt_head_ms = stats.expression_statement_head_ns / 1_000_000,
                identifier_ms = stats.identifier_ns / 1_000_000,
                build_ms = stats.build_ns / 1_000_000,
                owners_total = stats.owners_total,
                return_count = stats.return_count,
                throw_count = stats.throw_count,
                jsx_count = stats.jsx_count,
                expression_statement_count = stats.expression_statement_count,
                head_nodes_total = head.nodes,
                head_span_calls = head.span_calls,
                head_span_ms = head.span_ns / 1_000_000,
                "serialize_reference sub-phase totals",
            );
            out
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
