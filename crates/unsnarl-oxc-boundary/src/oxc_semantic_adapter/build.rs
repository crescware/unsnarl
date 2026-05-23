//! Top-level orchestration: drive `SemanticBuilder` and stitch the
//! per-entity mappings into a single [`ScopeAnalysisResult`].

use oxc_ast::ast::{Program, VariableDeclarationKind};
use oxc_ast::AstKind;
use oxc_index::IndexVec;
use oxc_semantic::SemanticBuilder;

use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::diagnostic_kind::DiagnosticKind;
use unsnarl_ir::ids::{DefinitionId, ScopeId};
use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::scope::DefinitionData;
use unsnarl_ir::IrArena;
use unsnarl_ir::Language;

use crate::analysis_result::ScopeAnalysisResult;
use crate::parser::SourceType;

use super::{definition_mapping, reference_mapping, scope_mapping, variable_mapping};

/// Adapter output bundle: the IR arena plus any diagnostics
/// (currently `VarDetected` warnings) collected during the build.
/// Returned to [`super::build_from_program`]'s caller so the boundary
/// can dispatch the diagnostics to its `AnalysisVisitor::on_diagnostic`
/// callback.
pub struct BuildOutput {
    pub analysis: ScopeAnalysisResult,
    pub diagnostics: Vec<Diagnostic>,
}

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
    language: Language,
    raw: &'a str,
) -> BuildOutput {
    let _span = tracing::info_span!("scope_build").entered();
    let ret = {
        let _span = tracing::info_span!("oxc_semantic").entered();
        SemanticBuilder::new().build(program)
    };
    let semantic = ret.semantic;
    let scope_mapping = {
        let _span = tracing::info_span!("scope_mapping").entered();
        scope_mapping::build_scopes(&semantic, source_type, language)
    };
    let mut scopes = scope_mapping.scopes;
    let translation = scope_mapping.translation;
    let switch_cases = scope_mapping.switch_cases;
    let mut definitions: IndexVec<DefinitionId, DefinitionData> = IndexVec::new();
    let variable_mapping = {
        let _span = tracing::info_span!("variable_mapping").entered();
        variable_mapping::build_variables(
            &semantic,
            &mut scopes,
            &mut definitions,
            &translation,
            &switch_cases,
        )
    };
    let mut variables = variable_mapping.variables;
    let symbol_to_variable = variable_mapping.symbol_to_variable;
    let synthetic_unresolved = variable_mapping.synthetic_unresolved;
    let inner_class_names = variable_mapping.inner_class_names;
    let references = {
        let _span = tracing::info_span!("reference_mapping").entered();
        reference_mapping::build_references(
            &semantic,
            &mut scopes,
            &mut variables,
            &mut definitions,
            &symbol_to_variable,
            &translation,
            &synthetic_unresolved,
            &switch_cases,
            &inner_class_names,
        )
    };
    {
        let _span = tracing::info_span!("definition_mapping").entered();
        definition_mapping::build_definitions(
            &semantic,
            &mut variables,
            &mut definitions,
            &symbol_to_variable,
        );
    }
    let diagnostics = {
        let _span = tracing::info_span!("collect_var_detected").entered();
        collect_var_detected_diagnostics(&semantic, raw)
    };
    BuildOutput {
        analysis: ScopeAnalysisResult {
            arena: IrArena {
                scopes,
                variables,
                references,
                definitions,
            },
            global_scope: ScopeId::from_usize(0),
        },
        diagnostics,
    }
}

/// Walk every `VariableDeclaration` AST node and emit a
/// `DiagnosticKind::VarDetected` warning for each `var` declaration.
///
/// `for (var ...; ...)`'s init and `for (var ... in/of ...)`'s left
/// slot are both ordinary `VariableDeclaration` nodes on the AST, so
/// a single walk covers every site without double-counting.
fn collect_var_detected_diagnostics(
    semantic: &oxc_semantic::Semantic<'_>,
    raw: &str,
) -> Vec<Diagnostic> {
    let index = SourceIndex::build(raw);
    let mut out = Vec::new();
    for node in semantic.nodes().iter() {
        let AstKind::VariableDeclaration(decl) = node.kind() else {
            continue;
        };
        if !matches!(decl.kind, VariableDeclarationKind::Var) {
            continue;
        }
        out.push(Diagnostic {
            kind: DiagnosticKind::VarDetected,
            message: "var declaration detected; rendered as node only (no edges).".to_string(),
            span: index.span_at(decl.span.start as usize),
        });
    }
    out
}

#[cfg(test)]
#[path = "build_test.rs"]
mod build_test;
