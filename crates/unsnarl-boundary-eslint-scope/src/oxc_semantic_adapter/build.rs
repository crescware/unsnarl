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

use super::{definition_mapping, reference_mapping, scope_mapping, variable_mapping};

/// Phase 2 entry point. Wires the scope / variable / reference /
/// definition mapping passes into the arena.
///
/// `reference_mapping` runs before `definition_mapping` because it
/// owns the `ImplicitGlobalVariable` synthesis path (one
/// `DefinitionData` per implicit-global Variable, emitted as a side
/// effect of resolving unresolved references). `definition_mapping`
/// then appends the remaining six `DefinitionType` variants
/// (`Variable` / `FunctionName` / `ClassName` / `Parameter` /
/// `CatchClause` / `ImportBinding`) by walking
/// `Scoping::symbol_declarations`.
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
    definition_mapping::build_definitions(
        &semantic,
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
