//! Top-level orchestration: drive `SemanticBuilder` and stitch the
//! per-entity mappings into a single [`EslintScopeAnalysisResult`].

use oxc_ast::ast::Program;
use oxc_index::IndexVec;
use oxc_semantic::SemanticBuilder;

use unsnarl_ir::ids::{DefinitionId, ScopeId};
use unsnarl_ir::scope::DefinitionData;
use unsnarl_ir::IrArena;
use unsnarl_ir::Language;

use crate::analysis_result::EslintScopeAnalysisResult;
use crate::parser::SourceType;

use super::{reference_mapping, scope_mapping, variable_mapping};

/// Phase 2 entry point. Wires the scope / variable / reference
/// mapping passes into the arena. `definitions` is seeded here as
/// an empty `IndexVec`; `reference_mapping` extends it with
/// `ImplicitGlobalVariable` rows for every unresolved reference, and
/// `definition_mapping` (Phase 2, follow-up commit) will append the
/// remaining six `DefinitionType` variants.
pub(crate) fn build<'a>(
    program: &Program<'a>,
    source_type: SourceType,
    _language: Language,
    _raw: &'a str,
) -> EslintScopeAnalysisResult {
    let ret = SemanticBuilder::new().build(program);
    let semantic = ret.semantic;
    let mut scopes = scope_mapping::build_scopes(&semantic, source_type);
    let (mut variables, symbol_to_variable) =
        variable_mapping::build_variables(&semantic, &mut scopes);
    let mut definitions: IndexVec<DefinitionId, DefinitionData> = IndexVec::new();
    let references = reference_mapping::build_references(
        &semantic,
        &mut scopes,
        &mut variables,
        &mut definitions,
        &symbol_to_variable,
    );
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
