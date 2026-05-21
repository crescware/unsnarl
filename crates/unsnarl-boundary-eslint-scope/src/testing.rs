//! Build helpers shared by the boundary-crate `*_test.rs` files.
//!
//! Mirrors `ts/src/boundary/eslint-scope/testing/`. The TS layer
//! offers `parse(code, language)` and `findFirst(root, type)`; the
//! Rust port replaces both with a higher-level helper
//! ([`analyze_source`]) that drives `OxcParser` → `analyze` and
//! returns the populated [`EslintScopeAnalysisResult`], plus a
//! handful of shared IR-shape predicates used by the sibling
//! `*_test.rs` files.
//!
//! Per issue #118, boundary tests stay integration-style — source
//! string in, IR observation out — so individual `enter_*` / classify
//! helpers don't need `&'a Program<'a>` mocks. The TS-side
//! `findFirst` (used to pull AST sub-nodes for unit tests) has no
//! Rust counterpart for that reason.

#![cfg(test)]

use oxc_allocator::Allocator;

use unsnarl_ir::ids::ScopeId;
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::DefinitionType;
use unsnarl_ir::IrArena;
use unsnarl_ir::Language;

use crate::analysis_result::EslintScopeAnalysisResult;
use crate::analyze::{analyze, AnalyzeOptions};
use crate::parser::{default_source_type_for, OxcParser, ParseOptions};
use crate::visitor::AnalysisVisitor;

pub(crate) struct NoopVisitor;
impl AnalysisVisitor for NoopVisitor {}

/// Parse `code` as the requested language and run the full
/// scope-builder pass against it.
pub(crate) fn analyze_source(code: &str, language: Language) -> EslintScopeAnalysisResult {
    let allocator = Allocator::default();
    let parsed = OxcParser
        .parse(
            &allocator,
            code,
            &ParseOptions {
                language,
                source_path: format!("input.{}", language_extension(language)),
                source_type: default_source_type_for(language),
            },
        )
        .expect("test source must parse cleanly");
    let mut visitor = NoopVisitor;
    analyze(
        &parsed.program,
        &AnalyzeOptions {
            source_type: parsed.source_type,
            language,
            raw: parsed.raw,
        },
        &mut visitor,
    )
}

fn language_extension(language: Language) -> &'static str {
    match language {
        Language::Js => "js",
        Language::Jsx => "jsx",
        Language::Ts => "ts",
        Language::Tsx => "tsx",
    }
}

/// Variable names live in a scope, ordered by `variables` insertion.
pub(crate) fn variable_names_in_scope(arena: &IrArena, scope: ScopeId) -> Vec<String> {
    arena.scopes[scope]
        .variables
        .iter()
        .map(|&id| arena.variables[id].name().to_string())
        .collect()
}

/// First child of `root` matching `target` scope type. Useful for
/// pulling the function / class / catch scope out of the global
/// scope's `child_scopes` without having to count indices.
pub(crate) fn find_first_descendant_scope(
    arena: &IrArena,
    root: ScopeId,
    target: ScopeType,
) -> Option<ScopeId> {
    if arena.scopes[root].r#type == target {
        return Some(root);
    }
    for &child in &arena.scopes[root].child_scopes {
        if let Some(hit) = find_first_descendant_scope(arena, child, target) {
            return Some(hit);
        }
    }
    None
}

pub(crate) fn variable_has_def_of(arena: &IrArena, name: &str, kind: DefinitionType) -> bool {
    arena
        .variables
        .iter()
        .filter(|v| v.name() == name)
        .flat_map(|v| v.defs.iter())
        .any(|&d| arena.definitions[d].r#type == kind)
}
