import type { VisualEdge } from "../visual-edge.js";
import type { VisualNode } from "../visual-node.js";
import type { VisualSubgraph } from "../visual-subgraph.js";

export type PendingLoopTestAnchor = Readonly<{
  subgraph: VisualSubgraph;
  node: VisualNode;
  position: "first" | "last";
}>;

export type BuildState = Readonly<{
  subgraphByScope: Map<string, VisualSubgraph>;
  functionSubgraphByFn: Map<string, VisualSubgraph>;
  returnSubgraphsByFn: Map<string, Map<string, VisualSubgraph>>;
  returnUseAdded: Set<string>;
  throwSubgraphsByFn: Map<string, Map<string, VisualSubgraph>>;
  throwUseAdded: Set<string>;
  ifTestAnchorByOffset: Map<number, string>;
  switchDiscriminantAnchorByOffset: Map<number, string>;
  whileTestAnchorByOffset: Map<number, string>;
  doWhileTestAnchorByOffset: Map<number, string>;
  forTestAnchorByOffset: Map<number, string>;
  pendingLoopTestAnchors: /* mutable */ PendingLoopTestAnchor[];
  expressionStatementByOffset: Map<number, string>;
  emittedEdges: Set<string>;
  edges: /* mutable */ VisualEdge[];
  // Maps any scope id (a collapsed scope itself or any descendant scope of
  // one) to the *root* collapsed scope id -- i.e. the topmost collapsed
  // ancestor. Used to discover the anchor for edge redirection. Optional
  // so unit-test fixtures can omit it; build-visual-graph always populates
  // it.
  collapsedRootByScope?: Map<string, string>;
  // Maps a root collapsed scope id to the node id of the variable that
  // owns it (e.g. `fnB` for `const fnB = (arr) => ...`). Edges that would
  // cross the collapsed boundary redirect to this anchor; if no anchor is
  // recorded for the collapsed scope (anonymous callbacks, branch bodies,
  // bare blocks), the redirect falls back to the closest visible
  // ancestor subgraph (see suppressedPredicateRedirect for the
  // predicate-anchor counterpart) instead of being dropped silently.
  collapsedAnchorByRoot?: Map<string, string>;
  // Maps a control-statement offset (if / for / while / do-while /
  // switch) to the BeyondDepth stub id its predicate-anchor refs should
  // land on. Recorded when the gated body collapses past the depth
  // threshold, so the test anchor of that statement cannot be created.
  // Refs whose predicate pointed at the now-missing anchor (e.g. `f` in
  // `if (f) { ... }` where the `if (f)` body collapsed) land here
  // instead of dangling off into module_root.
  suppressedPredicateRedirect?: Map<number, string>;
  // One BeyondDepth stub per visible ancestor subgraph. All anonymous
  // collapsed subtrees that share the same surviving outer container
  // funnel into the same `((...))` placeholder, so the rendered graph
  // shows a single boundary marker per visible parent rather than one
  // per hidden child.
  beyondDepthStubByParent?: Map<string, string>;
  // Maps every emitted node id back to the scope id whose contents
  // produced it. Used during edge post-processing to decide whether an
  // endpoint lives inside a collapsed scope and should be redirected to
  // the placeholder. Test anchors (if/switch/while/...) are excluded
  // here because they are part of the surrounding control subgraph
  // itself, not of an inner scope. Optional for the same reason as above.
  nodeIdOriginScope?: Map<string, string>;
}>;
