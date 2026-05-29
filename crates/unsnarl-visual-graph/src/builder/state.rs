//! Subgraph-keyed maps store [`SubgraphIdx`] handles into a
//! [`BuildArena`](super::arena::BuildArena) so the same subgraph can
//! be looked up from many sites without juggling overlapping `&mut`
//! references. The string-id maps (`if_test_anchor_by_offset`,
//! `expression_statement_by_offset`, `beyond_depth_stub_by_parent`,
//! ...) carry plain `String` node ids because their consumers
//! (`predicate_target_id`, `ensure_*`, edge redirection) want the
//! id directly rather than another arena handle.
//!
//! Every map / set is kept non-optional because `build_visual_graph`
//! always populates them at the call site that matters.

use std::collections::{HashMap, HashSet};

use crate::visual_edge::VisualEdge;

use super::arena::{NodeIdx, SubgraphIdx};

/// Where the pending loop-test / switch-discriminant anchor must
/// land inside its subgraph at the end of the build.
///
/// `First`: `subgraph.elements.unshift(node)` (for-test /
/// while-test / switch-discriminant).
///
/// `Last`: `subgraph.elements.push(node)` (do-while-test).
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum LoopTestAnchorPosition {
    First,
    Last,
}

/// Deferred test-anchor placement. The anchor node is created when
/// its body subgraph is built, but the actual `unshift` / `push` is
/// applied at the very end so other passes (variables, write ops,
/// children) populate the subgraph first and the anchor sits at the
/// correct edge of the final element list.
#[derive(Clone, Copy, Debug)]
pub struct PendingLoopTestAnchor {
    pub subgraph: SubgraphIdx,
    pub node: NodeIdx,
    pub position: LoopTestAnchorPosition,
}

#[derive(Default)]
pub struct BuildState {
    /// `scope.id → SubgraphIdx`. Populated by `build_scope` when a
    /// subgraph is created for that scope. Lookups feed
    /// `find_host_subgraph`, `visible_ancestor_subgraph`, and the
    /// if-test anchor placement in `build_children`.
    pub subgraph_by_scope: HashMap<String, SubgraphIdx>,
    /// `owner var id → SubgraphIdx`. Each function subgraph is keyed
    /// by the variable that introduces it (e.g. `fnB` for `function
    /// fnB() {}`) so `find_host_subgraph` can fall back to the
    /// owning function when no scope-direct hit is found.
    pub function_subgraph_by_fn: HashMap<String, SubgraphIdx>,
    /// `host key → completion span key → SubgraphIdx`. The host key
    /// is the owner var id when the enclosing function is owned by a
    /// variable, or the host scope id (`find_host_scope_id`) for an
    /// owner-var-less callback. Tracks the per-function Return
    /// subgraph created on first `ensure_return_use_node` hit so
    /// subsequent returns inside the same statement reuse the same
    /// wrapping subgraph.
    pub return_subgraphs_by_fn: HashMap<String, HashMap<String, SubgraphIdx>>,
    /// Set of ref ids whose return-use node has already been
    /// emitted. Prevents duplicate nodes when the same ref is
    /// visited from multiple call paths.
    pub return_use_added: HashSet<String>,
    /// Same as `return_subgraphs_by_fn` (including the owner-var /
    /// host-scope-id host key) but for `throw` completions.
    pub throw_subgraphs_by_fn: HashMap<String, HashMap<String, SubgraphIdx>>,
    /// Same as `return_use_added` but for `throw` completions.
    pub throw_use_added: HashSet<String>,
    /// `if-statement offset → test anchor node id`. Filled by the
    /// if-test push in `build_children` so `predicate_target_id`
    /// can route reads to the corresponding `if-test` node.
    pub if_test_anchor_by_offset: HashMap<u32, String>,
    /// `switch-statement offset → discriminant anchor node id`.
    pub switch_discriminant_anchor_by_offset: HashMap<u32, String>,
    /// `while-statement offset → test anchor node id`.
    pub while_test_anchor_by_offset: HashMap<u32, String>,
    /// `do-while-statement offset → test anchor node id`.
    pub do_while_test_anchor_by_offset: HashMap<u32, String>,
    /// `for-statement offset → header anchor node id`. The for
    /// statement family (`for` / `for-in` / `for-of`) shares this
    /// map; the anchor kind discriminates them.
    pub for_test_anchor_by_offset: HashMap<u32, String>,
    /// Anchors that must be `unshift` / `push`ed into their host
    /// subgraph at the very end of the build, after every other
    /// element has been added.
    pub pending_loop_test_anchors: Vec<PendingLoopTestAnchor>,
    /// `expression-statement offset → synthetic-expression-stmt
    /// node id`. `ensure_expression_statement_node` consults this
    /// to dedupe per statement.
    pub expression_statement_by_offset: HashMap<u32, String>,
    /// `from -->|label| to` key set. `push_edge` checks this before
    /// minting a fresh `VisualEdge`.
    pub emitted_edges: HashSet<String>,
    /// Owned `VisualEdge` list. Copied into `VisualGraph.edges` at
    /// the end of the build.
    pub edges: Vec<VisualEdge>,
    /// `descendant scope id → root collapsed scope id`. Identifies
    /// whether an endpoint scope (or any ancestor) was collapsed by
    /// the depth pass so edge redirection can route reads/writes
    /// to the visible anchor.
    pub collapsed_root_by_scope: HashMap<String, String>,
    /// `root collapsed scope id → anchor node id`. The anchor is
    /// the visible target (owner variable node or a BeyondDepth
    /// stub inside the closest visible ancestor subgraph) that all
    /// edges crossing the collapsed boundary land on.
    pub collapsed_anchor_by_root: HashMap<String, String>,
    /// `control statement offset → BeyondDepth stub id`. Records
    /// where a suppressed predicate anchor should redirect (refs
    /// reading `if (f)` where the `if` body collapsed).
    pub suppressed_predicate_redirect: HashMap<u32, String>,
    /// `visible ancestor subgraph id → BeyondDepth stub id`. One
    /// stub per visible parent so multiple anonymous collapsed
    /// children share the same `((...))` marker.
    pub beyond_depth_stub_by_parent: HashMap<String, String>,
    /// `node id → originating scope id`. Built incrementally as
    /// nodes are created so edge redirection can decide whether an
    /// endpoint lives inside a collapsed scope. Test anchors are
    /// intentionally absent because they belong to their
    /// surrounding control subgraph, not the inner gated scope.
    pub node_id_origin_scope: HashMap<String, String>,
}

impl BuildState {
    pub fn new() -> Self {
        Self::default()
    }
}
