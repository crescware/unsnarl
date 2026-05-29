//! Immutable side tables threaded through every builder helper:
//! variable / scope lookup maps borrowed from the `SerializedIR`,
//! plus the precomputed `WriteOp` indices that drive edge emission.
//! Map key shape (`&str` for IR-borrowed ids, owned `String` for
//! values that the builder constructs itself such as subgraph
//! owners and `WriteOp` ids) matches the convention already in use
//! by the other builder helpers (see e.g. `is_ancestor_scope`,
//! `branch_scope_of`).

use std::collections::HashMap;

use unsnarl_ir::nesting_kind::NestingDepths;
use unsnarl_ir::primitive::SourceIndex;
use unsnarl_ir::serialized::{SerializedIR, SerializedScope, SerializedVariable};

use super::expression_statement_index::ExpressionStatementIndex;
use super::write_op::WriteOp;

/// Optional knobs the caller hands to `build_visual_graph`.
#[derive(Default, Clone)]
pub struct BuildVisualGraphOptions {
    pub depths: Option<NestingDepths>,
}

pub struct BuilderContext<'a> {
    /// Borrowed `SerializedIR` so the builder can re-walk
    /// `ir.scopes` / `ir.variables` / `ir.references` / `ir.raw`
    /// directly. Lifetime `'a` ties every borrowed-key map below to
    /// the same IR.
    pub ir: &'a SerializedIR,
    /// `variable id → &SerializedVariable`. Keys borrow from the IR
    /// so no allocation per fixture.
    pub variable_map: HashMap<&'a str, &'a SerializedVariable>,
    /// `scope id → &SerializedScope`. Same shape as `variable_map`.
    pub scope_map: HashMap<&'a str, &'a SerializedScope>,
    /// `function-scope id → owner variable id`. Populated for
    /// scopes whose owning binding lives one level up
    /// (`function fnB` / `const fnB = () => {}`). Values are owned
    /// because the constructor of `BuilderContext` builds the map
    /// fresh; no zero-copy benefit here.
    pub subgraph_owner_var: HashMap<String, String>,
    /// `variable id → Write `WriteOp`s in identifier-offset order`.
    pub write_ops_by_variable: HashMap<String, Vec<WriteOp>>,
    /// `scope id → Write `WriteOp`s declared in that scope`.
    pub write_ops_by_scope: HashMap<String, Vec<WriteOp>>,
    /// `reference id → WriteOp` for direct ref-to-op lookup.
    pub write_op_by_ref: HashMap<String, WriteOp>,
    /// `branch-container key → cases sorted by source order`. Keys
    /// look like `switch:<parentScope>:<offset>`; the value lets
    /// fallthrough handling pick the textual "last case".
    pub sorted_cases_by_container: HashMap<String, Vec<&'a SerializedScope>>,
    /// `branch-container key → every scope sharing that key`. Built
    /// once per fixture by walking `ir.scopes`. `read_origins` uses
    /// it to fan out across the sibling branches of an `if` / `try` /
    /// `switch` container without re-filtering `ir.scopes` per
    /// reference. Order matches `ir.scopes`'s source order.
    pub branch_scopes_by_container: HashMap<String, Vec<&'a SerializedScope>>,
    /// Depth ceiling for the pruning pass. `None` means
    /// `is_collapsed` returns false for every scope and the full
    /// graph is rendered.
    pub depths: Option<NestingDepths>,
    /// Precomputed line-start / UTF-16 index over `ir.raw`. Lookups
    /// such as `line_for_offset(offset)` resolve in `O(log lines)`
    /// rather than re-scanning `raw` from byte 0 each call.
    pub source_index: SourceIndex<'a>,
    /// Where the (non-synthetic) `ExpressionStatement`s are. Consumed
    /// by `build_children` to correlate a callback function scope to
    /// the statement hosting its CallProxy wrapper (`enclosing`, span
    /// containment) and to render that wrapper's `callName` / span
    /// lines. See [`ExpressionStatementIndex`] for why this correlation
    /// lives in the visual layer rather than baked into the IR
    /// `callbackArgument`, and why synthetic arrow-body statements are
    /// absent from it.
    pub expression_statement_index: ExpressionStatementIndex<'a>,
}
