//! Builds a per-scope `NestingDepths` from the subgraph hierarchy that
//! the visual layer will actually render — the input the depth-pruning
//! pass consults via `is_collapsed`.
//!
//! The counter for a `NestingKind` is incremented at every scope that
//! (a) is materialised as its own subgraph (`should_subgraph`),
//! (b) classifies under that kind (`nesting_kind_of`), and
//! (c) is not a pair-member excluded by `depth_increment_kind` to
//!     avoid double-counting one user-visible nesting step.
//!
//! A single `for` / `catch` / `switch` produces *two* subgraph frames
//! (`for L1-13` → `block L1-13`, etc.). Counting both would tally one
//! syntactic step as two depth units. The exclusion rule asymmetrically
//! suppresses one side of each pair:
//!
//! * For `for` / `catch`: the `ScopeType::For` / `Catch` wrapper is
//!   suppressed; the body `Block` carries the increment (mirrors
//!   `compute_nesting_depths`, where `ForStatement` / `CatchClause`
//!   are visited but do not `inc`).
//! * For `switch`: the per-`SwitchCase` synthesised Block is
//!   suppressed; the `ScopeType::Switch` wrapper carries the
//!   increment. (Same reason — but inverted, because one `switch`
//!   can hold many cases; counting per case would conflate `case N;`
//!   with nesting depth.)
//!
//! The traversal is a DFS over the IR scope tree starting from the
//! module / global root, so synthesised subgraphs (notably the ternary
//! arms produced by `synthesise_conditional_arms`, which carry no oxc
//! scope but are inserted into the IR scope tree as `Block` scopes)
//! are counted identically to AST-anchored subgraphs. The decision
//! uses the visual layer's own subgraph / nesting predicates, not the
//! AST shape — that is the whole point: `is_collapsed` now reflects
//! the rendered hierarchy uniformly, instead of being gated by which
//! AST node types the analyzer's `compute_nesting_depths` happens to
//! recognise.
//!
//! Each scope's recorded snapshot is taken *after* its own potential
//! increment: a scope at the bottom of an `n`-deep chain of same-kind
//! subgraphs reports depth `n`, which is then compared against the
//! ceiling.

use std::collections::HashMap;

use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths, NestingKind};
use unsnarl_ir::scope_type::ScopeType;
use unsnarl_ir::serialized::{SerializedIR, SerializedScope};
use unsnarl_oxc_parity::AstType;

use super::nesting_kind_of::nesting_kind_of;
use super::should_subgraph::should_subgraph;

pub fn compute_rendered_nesting_depths(ir: &SerializedIR) -> HashMap<String, NestingDepths> {
    let mut scope_map: HashMap<&str, &SerializedScope> = HashMap::new();
    for s in &ir.scopes {
        scope_map.insert(s.id.value(), s);
    }
    let mut out: HashMap<String, NestingDepths> = HashMap::new();
    let mut counters = Counters::default();
    // Walk from every top-level scope. In real IR exactly one
    // `ScopeType::Module` or `Global` carries `upper == None`; sibling
    // unit tests sometimes hand us a partial tree whose root is a
    // detached function / block. Using `upper.is_none()` covers both,
    // so every scope reachable from the supplied IR ends up with a
    // recorded snapshot — the caller's `expect("…rendered nesting
    // depths are precomputed…")` cannot blow up on a synthetic tree.
    for s in &ir.scopes {
        if s.upper.is_none() {
            walk(s, &scope_map, &mut counters, &mut out);
        }
    }
    out
}

#[derive(Default)]
struct Counters {
    function: u32,
    r#if: u32,
    r#for: u32,
    r#while: u32,
    switch: u32,
    try_catch_finally: u32,
    block: u32,
}

impl Counters {
    fn snapshot(&self) -> NestingDepths {
        NestingDepths {
            function: NestingDepth(self.function),
            r#if: NestingDepth(self.r#if),
            r#for: NestingDepth(self.r#for),
            r#while: NestingDepth(self.r#while),
            switch: NestingDepth(self.switch),
            try_catch_finally: NestingDepth(self.try_catch_finally),
            block: NestingDepth(self.block),
        }
    }

    fn inc(&mut self, kind: NestingKind) {
        match kind {
            NestingKind::Function => self.function += 1,
            NestingKind::If => self.r#if += 1,
            NestingKind::For => self.r#for += 1,
            NestingKind::While => self.r#while += 1,
            NestingKind::Switch => self.switch += 1,
            NestingKind::TryCatchFinally => self.try_catch_finally += 1,
            NestingKind::Block => self.block += 1,
        }
    }

    fn dec(&mut self, kind: NestingKind) {
        match kind {
            NestingKind::Function => self.function -= 1,
            NestingKind::If => self.r#if -= 1,
            NestingKind::For => self.r#for -= 1,
            NestingKind::While => self.r#while -= 1,
            NestingKind::Switch => self.switch -= 1,
            NestingKind::TryCatchFinally => self.try_catch_finally -= 1,
            NestingKind::Block => self.block -= 1,
        }
    }
}

fn walk(
    scope: &SerializedScope,
    scope_map: &HashMap<&str, &SerializedScope>,
    counters: &mut Counters,
    out: &mut HashMap<String, NestingDepths>,
) {
    let inc_kind = depth_increment_kind(scope);
    if let Some(kind) = inc_kind {
        counters.inc(kind);
    }
    out.insert(scope.id.value().to_string(), counters.snapshot());
    for child_id in &scope.child_scopes {
        if let Some(child) = scope_map.get(child_id.value()).copied() {
            walk(child, scope_map, counters, out);
        }
    }
    if let Some(kind) = inc_kind {
        counters.dec(kind);
    }
}

/// Returns the `NestingKind` `scope` adds to the rendered depth, or
/// `None` when it does not bump any counter.
///
/// `None` covers three cases:
///
/// * the scope does not produce its own subgraph (`should_subgraph`
///   is false), or it does but classifies under no kind (e.g. the
///   module / global root, or a function-expression name scope);
/// * the scope is a `ScopeType::For` / `Catch` wrapper that pairs
///   with an inner body `Block` carrying the same kind — let the
///   body Block carry the increment (mirroring `compute_nesting_depths`,
///   where `ForStatement` / `CatchClause` are visited but do not
///   `inc` — only the body `BlockStatement` does); and
/// * the scope is a `SwitchCase` synthesised Block — the increment
///   sits on the `SwitchStatement` wrapper above it, so per-case
///   Blocks must not double-tally. (One `switch { case A; case B; }`
///   is one Switch step, not two.)
fn depth_increment_kind(scope: &SerializedScope) -> Option<NestingKind> {
    if !should_subgraph(scope) {
        return None;
    }
    if matches!(scope.r#type, ScopeType::For | ScopeType::Catch) {
        return None;
    }
    if matches!(scope.block.r#type, AstType::SwitchCase) {
        return None;
    }
    nesting_kind_of(scope)
}

#[cfg(test)]
#[path = "compute_rendered_nesting_depths_test.rs"]
mod compute_rendered_nesting_depths_test;
