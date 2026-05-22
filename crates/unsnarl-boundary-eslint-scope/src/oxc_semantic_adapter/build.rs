//! Top-level orchestration: drive `SemanticBuilder` and stitch the
//! per-entity mappings into a single [`EslintScopeAnalysisResult`].

use oxc_ast::ast::Program;
use oxc_index::IndexVec;
use oxc_semantic::SemanticBuilder;

use unsnarl_ir::ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{DefinitionData, VariableData};
use unsnarl_ir::IrArena;
use unsnarl_ir::Language;

use crate::analysis_result::EslintScopeAnalysisResult;
use crate::parser::SourceType;

use super::scope_mapping;

/// Phase 2 entry point. Currently wires the scope-mapping pass into
/// the arena; variable / reference / definition mappings still return
/// empty tables and will be wired in successive commits within Phase 2.
pub(crate) fn build<'a>(
    program: &Program<'a>,
    source_type: SourceType,
    _language: Language,
    _raw: &'a str,
) -> EslintScopeAnalysisResult {
    let ret = SemanticBuilder::new().build(program);
    let semantic = ret.semantic;
    let scopes = scope_mapping::build_scopes(&semantic, source_type);
    // TODO(phase-2): drive `variable_mapping`, `reference_mapping`,
    // `definition_mapping` here. The current stub returns empty
    // tables for those entities so the rest of the boundary can be
    // wired up incrementally without affecting `analyze()`'s real
    // output.
    let variables: IndexVec<VariableId, VariableData> = IndexVec::new();
    let references: IndexVec<ReferenceId, ReferenceData> = IndexVec::new();
    let definitions: IndexVec<DefinitionId, DefinitionData> = IndexVec::new();
    EslintScopeAnalysisResult {
        arena: IrArena {
            scopes,
            variables,
            references,
            definitions,
        },
        global_scope: ScopeId::from_usize(0),
    }
}
