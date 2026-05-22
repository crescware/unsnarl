//! Top-level orchestration: drive `SemanticBuilder` and stitch the
//! per-entity mappings into a single [`EslintScopeAnalysisResult`].

use oxc_ast::ast::Program;
use oxc_index::IndexVec;
use oxc_semantic::SemanticBuilder;

use unsnarl_ir::ids::{DefinitionId, ReferenceId, ScopeId, VariableId};
use unsnarl_ir::reference::ReferenceData;
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};
use unsnarl_ir::IrArena;
use unsnarl_ir::Language;

use crate::analysis_result::EslintScopeAnalysisResult;

/// Phase 2 entry point. Currently returns an empty arena that pins the
/// scope tree's root to the program but contains no other rows; the
/// per-entity mappings (scopes / variables / references / definitions)
/// will be wired in successive commits within Phase 2.
pub(crate) fn build<'a>(
    program: &Program<'a>,
    _language: Language,
    _raw: &'a str,
) -> EslintScopeAnalysisResult {
    let ret = SemanticBuilder::new().build(program);
    let _semantic = ret.semantic;
    // TODO(phase-2): drive `scope_mapping`, `variable_mapping`,
    // `reference_mapping`, `definition_mapping` here. The current
    // stub returns an empty arena so the rest of the boundary can be
    // wired up incrementally without affecting `analyze()`'s real
    // output.
    let scopes: IndexVec<ScopeId, ScopeData> = IndexVec::new();
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
