import type { VisualEdge } from "../../visual-graph/model.js";

export function collectEdgeEndpointIds(
  edges: ReadonlyArray<VisualEdge>,
): Set<string> {
  const ids = new Set<string>();
  for (const e of edges) {
    ids.add(e.from);
    ids.add(e.to);
  }
  return ids;
}
