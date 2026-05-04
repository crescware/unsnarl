import type { VisualEdge } from "../../visual-graph/visual-edge.js";

export function collectEdgeEndpointIds(
  edges: readonly VisualEdge[],
): Set<string> {
  const ids = new Set<string>();
  for (const e of edges) {
    ids.add(e.from);
    ids.add(e.to);
  }
  return ids;
}
