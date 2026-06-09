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
    /// `conditional-expression offset → test anchor node id`. Filled
    /// by the ternary test push in `build_children` so
    /// `predicate_target_id` can route reads to the corresponding
    /// `ternary ?:` diamond node.
    pub conditional_test_anchor_by_offset: HashMap<u32, String>,
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
    /// `return-span key ("start-end") → CallProxy id`, for a
    /// `return <call>(cb)`. The return completion's inputs route to this
    /// proxy instead of minting a return-use node, so the returned
    /// call's callback is not stranded as a disconnected island.
    pub return_proxy_by_span: HashMap<String, String>,
    /// `return-span key → consequent/alternate arm source span`, the
    /// `return c ? items.map(cb) : other` counterpart of
    /// `result_proxy_arm_span`. Both arms of the ternary share the one
    /// return completion span, so without gating the sibling arm's value
    /// (`other`) would route to the call's return proxy too. When present
    /// the proxy claims only reads inside the arm hosting the call; the
    /// sibling arm falls through to its own return-use node.
    pub return_proxy_arm_span: HashMap<String, (u32, u32)>,
    /// `result variable id → result-bound CallProxy id`, for
    /// `const xs = arr.map(cb)`. The call's inputs are redirected from
    /// the `xs` node to the proxy, so the dataflow backbone reads
    /// `input → the call → xs` instead of the inputs pointing straight
    /// at `xs`.
    pub result_proxy_by_var: HashMap<String, String>,
    /// `result variable id → consequent/alternate arm source span` for a
    /// CallProxy that lives inside a ternary arm
    /// (`const xs = c ? items.map(cb) : other`). A ternary binds its
    /// variable from *two* sources — the call in one arm and the other
    /// arm's value — but `result_proxy_by_var` would redirect every read
    /// owning `xs` to the call's proxy, wrongly pulling the sibling arm's
    /// value (`other`) through the call. When an entry is present here the
    /// redirect is gated to reads whose offset falls inside the arm that
    /// hosts the call, so the sibling arm flows to `xs` directly. Absent
    /// for ordinary (non-ternary) bindings, where the call is the sole
    /// source and the unconditional redirect is correct.
    pub result_proxy_arm_span: HashMap<String, (u32, u32)>,
    /// `write-op node id → reassignment-bound CallProxy id`, the
    /// assignment counterpart of `result_proxy_by_var` for
    /// `y = arr.map(cb)`. Keyed on the reassignment's write-op node
    /// because the result variable's own node lives at its declaration
    /// site, elsewhere in the graph; the same `input → the call → write`
    /// redirection then applies.
    pub result_proxy_by_write_op: HashMap<String, String>,
    /// `write-op node id → consequent/alternate arm source span`, the
    /// reassignment (`y = c ? items.map(cb) : other`) counterpart of
    /// `result_proxy_arm_span`. Gates the write-op proxy redirect to the
    /// arm hosting the call so the sibling arm's value keeps its edge to
    /// the write-op node.
    pub result_proxy_write_op_arm_span: HashMap<String, (u32, u32)>,
    /// Source spans of ternary arms that host a statement-level CallProxy
    /// for an arm callback (`enabled ? items.map(cb) : other;`, value
    /// discarded). The arm's callback receiver routes to that proxy, but
    /// a ternary arm's plain value reads (the sibling arm, or any
    /// non-call arm) must not be pulled onto the statement's container —
    /// they flow to the ternary's consumer (here the module sink). A read
    /// inside a ternary arm is routed to the statement container only when
    /// its offset falls in one of these hosting-arm spans.
    pub ternary_callback_arm_spans: Vec<(u32, u32)>,
}

impl BuildState {
    pub fn new() -> Self {
        Self::default()
    }
}
