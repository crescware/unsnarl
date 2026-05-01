import type { VisualEdge, VisualSubgraph } from "../model.js";

export type BuildState = Readonly<{
  subgraphByScope: Map<string, VisualSubgraph>;
  functionSubgraphByFn: Map<string, VisualSubgraph>;
  returnSubgraphsByFn: Map<string, Map<string, VisualSubgraph>>;
  returnUseAdded: Set<string>;
  emittedEdges: Set<string>;
  edges: VisualEdge[];
}>;
