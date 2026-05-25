//! Build helpers shared by the boundary-crate `*_test.rs` files.
//!
//! [`analyze_source`] is the workhorse: it drives `OxcParser` →
//! `analyze` and returns the populated
//! [`ScopeAnalysisResult`]. Companion helpers expose shared
//! IR-shape predicates used by the sibling `*_test.rs` files.
//!
//! Boundary tests stay integration-style — source string in, IR
//! observation out — so individual `enter_*` / classify helpers
//! don't need `&'a Program<'a>` mocks.

#![cfg(test)]

use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::ids::{ScopeId, VariableId};
use unsnarl_ir::DefinitionType;
use unsnarl_ir::IrArena;
use unsnarl_ir::Language;

use crate::analysis_result::ScopeAnalysisResult;
use crate::analyze::parse_and_analyze_with;
use crate::parser::{default_source_type_for, SourceType};
use crate::visitor::AnalysisVisitor;

/// Captures every diagnostic the scope-builder emits during
/// `analyze`. Diagnostics flow through `AnalysisVisitor::on_diagnostic`,
/// so a per-test visitor is the natural observation point.
pub(crate) struct DiagnosticCapturingVisitor {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticCapturingVisitor {
    pub(crate) fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }
}

impl AnalysisVisitor for DiagnosticCapturingVisitor {
    fn on_diagnostic(&mut self, diag: &Diagnostic) {
        self.diagnostics.push(diag.clone());
    }
}

/// Parse `code` as the requested language and run the full
/// scope-builder pass against it.
pub(crate) fn analyze_source(code: &str, language: Language) -> ScopeAnalysisResult {
    analyze_source_as(code, language, default_source_type_for(language))
}

/// Parse `code` with an explicit `source_type` override; used to
/// assert module-vs-script branching independently of the language
/// tag.
pub(crate) fn analyze_source_as(
    code: &str,
    language: Language,
    source_type: SourceType,
) -> ScopeAnalysisResult {
    parse_and_analyze_with(
        code,
        language,
        source_type,
        &mut crate::analyze::NoopVisitor,
    )
    .expect("test source must parse cleanly")
}

/// Run the scope-builder and return both the analysis result and the
/// list of diagnostics surfaced by the visitor.
pub(crate) fn analyze_source_with_diagnostics(
    code: &str,
    language: Language,
) -> (ScopeAnalysisResult, Vec<Diagnostic>) {
    let mut visitor = DiagnosticCapturingVisitor::new();
    let result = parse_and_analyze_with(
        code,
        language,
        default_source_type_for(language),
        &mut visitor,
    )
    .expect("test source must parse cleanly");
    (result, visitor.diagnostics)
}

/// Variable names live in a scope, ordered by `variables` insertion.
pub(crate) fn variable_names_in_scope(arena: &IrArena, scope: ScopeId) -> Vec<String> {
    arena.scopes[scope]
        .variables
        .iter()
        .map(|&id| arena.variables[id].name().to_string())
        .collect()
}

/// Locate a variable inside `scope` by name. Returns the `VariableId`
/// from `scope.set` (the same lookup the analyzer uses when resolving
/// declarations).
pub(crate) fn find_variable_in_scope(
    arena: &IrArena,
    scope: ScopeId,
    name: &str,
) -> Option<VariableId> {
    arena.scopes[scope].set().get(name).copied()
}

/// Materialise the list of definition types attached to a variable.
pub(crate) fn def_types_of(arena: &IrArena, variable: VariableId) -> Vec<DefinitionType> {
    arena.variables[variable]
        .defs
        .iter()
        .map(|&d| arena.definitions[d].r#type)
        .collect()
}

/// Assert that `variable` carries exactly one definition of the
/// given kind. Used in lieu of `assert_eq!(def_types_of(...), vec![T])`
/// because `DefinitionType` does not derive `Debug` (its serialized
/// form is the only public surface), so `assert_eq!` on a `Vec` of
/// them does not compile.
#[track_caller]
pub(crate) fn assert_single_def_type(
    arena: &IrArena,
    variable: VariableId,
    expected: DefinitionType,
) {
    let types = def_types_of(arena, variable);
    assert_eq!(
        types.len(),
        1,
        "expected exactly one def, got {} defs",
        types.len()
    );
    assert!(types[0] == expected, "definition type mismatch");
}

/// Pre-order traversal of a scope tree. Used to find catch / class
/// scopes by type without walking child-scope indices by hand.
pub(crate) fn collect_all_scopes(arena: &IrArena, root: ScopeId) -> Vec<ScopeId> {
    let mut out = Vec::new();
    walk_scope(arena, root, &mut out);
    out
}

fn walk_scope(arena: &IrArena, scope: ScopeId, out: &mut Vec<ScopeId>) {
    out.push(scope);
    for &child in &arena.scopes[scope].child_scopes {
        walk_scope(arena, child, out);
    }
}
