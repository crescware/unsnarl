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
  ifTestAnchorByOffset: Map<number, string>;
  whileTestAnchorByOffset: Map<number, string>;
  doWhileTestAnchorByOffset: Map<number, string>;
  forTestAnchorByOffset: Map<number, string>;
  pendingLoopTestAnchors: /* mutable */ PendingLoopTestAnchor[];
  expressionStatementByOffset: Map<number, string>;
  emittedEdges: Set<string>;
  edges: /* mutable */ VisualEdge[];
}>;
