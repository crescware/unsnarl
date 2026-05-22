//! Pipeline entry that turns a parsed source into an [`AnalyzedSource`].
//!
//! Three phases run end-to-end:
//!
//! 1. Compute the per-offset `NestingDepths` table.
//! 2. Drive the eslint-scope-compatible scope build through
//!    [`unsnarl_boundary_eslint_scope::analyze::analyze`] with a
//!    diagnostics-only `AnalysisVisitor`, then build span -> arena ID
//!    indices on the arena it returns.
//! 3. Walk the `Program` again with [`BuildAnalysisVisitor`] to fill
//!    the per-scope / per-reference side-table annotations, and
//!    iterate the arena directly to fill the per-variable `is_unused`
//!    flag.
//!
//! The IR is kept strictly lifetime-free (`AstNode = { type, span }`
//! only), so several analyzer functions need a separate walk that
//! retains `AstKind<'a>` handles in their ancestor chain. See
//! [`BuildAnalysisVisitor`]'s module doc for the per-function
//! breakdown.

use std::collections::HashMap;

use oxc_ast::ast::Program;
use oxc_ast_visit::Visit;

use unsnarl_boundary_eslint_scope::analyze::{analyze, AnalyzeOptions};
use unsnarl_boundary_eslint_scope::parser::SourceType;
use unsnarl_boundary_eslint_scope::visitor::AnalysisVisitor;
use unsnarl_ir::diagnostic::Diagnostic;
use unsnarl_ir::ids::{ReferenceId, ScopeId};
use unsnarl_ir::IrArena;
use unsnarl_ir::Language;
use unsnarl_oxc_parity::AstType;

use crate::analyzed_source::AnalyzedSource;
use crate::annotations_impl::AnnotationsImpl;
use crate::build_analysis_visitor::BuildAnalysisVisitor;
use crate::compute_nesting_depths::compute_nesting_depths;
use crate::is_unused::is_unused;

pub fn run_analysis<'a>(
    program: &Program<'a>,
    source_type: SourceType,
    language: Language,
    raw: &'a str,
) -> AnalyzedSource<'a> {
    let nesting_depths = compute_nesting_depths(program);

    let mut collector = DiagnosticCollector::default();
    let result = analyze(
        program,
        &AnalyzeOptions {
            source_type,
            language,
            raw,
        },
        &mut collector,
    );
    let diagnostics = collector.diagnostics;

    let mut span_to_scope: HashMap<(u32, u32), ScopeId> =
        HashMap::with_capacity(result.arena.scopes.len());
    for (id, scope) in result.arena.scopes.iter_enumerated() {
        span_to_scope.insert((scope.block.span.start, scope.block.span.end), id);
    }
    let mut span_to_ref: HashMap<(u32, u32), ReferenceId> =
        HashMap::with_capacity(result.arena.references.len());
    for (id, reference) in result.arena.references.iter_enumerated() {
        span_to_ref.insert(
            (
                reference.identifier.span.start,
                reference.identifier.span.end,
            ),
            id,
        );
    }

    let mut annotations = AnnotationsImpl::new();
    // The boundary stamps the normalised hashbang/directive/body
    // offset onto the global scope's `block.span.start` (see
    // `analyze::analyze`). Forward the same value to the side-table
    // walk so `block_context_of` / `find_predicate_container` see
    // `Program.span.start == normalised_start` instead of the raw
    // oxc value of `0`.
    let program_normalised_start = result.arena.scopes[result.global_scope].block.span.start;
    let mut walker = BuildAnalysisVisitor::new(
        raw,
        &result.arena,
        &mut annotations,
        &nesting_depths,
        &span_to_scope,
        &span_to_ref,
        program_normalised_start,
    );
    walker.visit_program(program);

    populate_variable_annotations(&result.arena, &mut annotations);

    AnalyzedSource {
        arena: result.arena,
        root_scope: result.global_scope,
        annotations,
        diagnostics,
        raw,
    }
}

#[derive(Default)]
struct DiagnosticCollector {
    diagnostics: Vec<Diagnostic>,
}

impl AnalysisVisitor for DiagnosticCollector {
    fn on_diagnostic(&mut self, diag: &Diagnostic) {
        self.diagnostics.push(diag.clone());
    }
}

fn populate_variable_annotations(arena: &IrArena, annotations: &mut AnnotationsImpl) {
    for (var_id, _) in arena.variables.iter_enumerated() {
        let is_unused_flag = is_unused(var_id, arena, |def_id| {
            let def = &arena.definitions[def_id];
            let init = def.init.as_ref()?;
            if is_functionlike(&init.r#type) {
                Some(init.span)
            } else {
                None
            }
        });
        annotations.set_variable(
            var_id,
            unsnarl_annotations::VariableAnnotation {
                is_unused: is_unused_flag,
            },
        );
    }
}

fn is_functionlike(ty: &AstType) -> bool {
    matches!(
        ty,
        AstType::FunctionDeclaration
            | AstType::FunctionExpression
            | AstType::ArrowFunctionExpression
            | AstType::ClassDeclaration
            | AstType::ClassExpression
    )
}
