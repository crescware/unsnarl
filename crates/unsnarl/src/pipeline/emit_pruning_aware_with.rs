//! Shared emit path for the visual-graph-aware emitters (json /
//! mermaid / markdown / stats): serialize, optionally prune / highlight,
//! then render with the given emitter.

use unsnarl_emitter::{EmitOptions, Emitter};
use unsnarl_ir::Language;
use unsnarl_oxc_boundary::parser::ParseError;

use crate::pipeline::plugin::apply_plugins;

use super::prepare_emit::{prepare_emit, PreparedEmit};
use super::serialize_ir::serialize_ir;
use super::{PipelineRunDetails, PipelineRunOptions};

#[derive(Clone, Copy)]
pub(super) struct EmitOptionsBase {
    pub(super) pretty_json: bool,
    pub(super) debug: bool,
}

pub(super) fn emit_pruning_aware_with(
    code: &str,
    source_path: &str,
    language: Language,
    emitter: &dyn Emitter,
    run: PipelineRunOptions<'_>,
    base_opts: EmitOptionsBase,
) -> Result<PipelineRunDetails, ParseError> {
    let serialized = apply_plugins(serialize_ir(code, source_path, language)?, run.plugins);
    let diagnostics = serialized.diagnostics.clone();
    let needs_visual =
        run.pruning.map(|p| !p.roots.is_empty()).unwrap_or(false) || run.highlight.is_some();
    let prepared = if needs_visual {
        prepare_emit(&serialized, run.pruning, run.depths, run.highlight)
    } else {
        PreparedEmit {
            pruned_graph: None,
            resolutions: None,
            per_query: None,
            highlight_ids: None,
            highlight_point_ids: None,
            highlight_warnings: None,
            highlight: None,
        }
    };
    let resolutions_for_details = prepared.resolutions.clone();
    let per_query_for_details = prepared.per_query;
    let highlight_warnings_for_details = prepared.highlight_warnings;
    let text = {
        let _span = unsnarl_instrumentation::span!("emit");
        emitter.emit(
            &serialized,
            &EmitOptions {
                pretty_json: base_opts.pretty_json,
                debug: base_opts.debug,
                pruned_graph: prepared.pruned_graph,
                resolutions: prepared.resolutions,
                depths: run.depths.cloned(),
                highlight_ids: prepared.highlight_ids,
                highlight_point_ids: prepared.highlight_point_ids,
                highlight: prepared.highlight,
            },
        )
    };
    Ok(PipelineRunDetails {
        text,
        pruning: per_query_for_details,
        resolutions: resolutions_for_details,
        highlight_warnings: highlight_warnings_for_details,
        diagnostics,
    })
}
