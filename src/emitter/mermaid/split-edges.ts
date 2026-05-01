import type { VisualEdge } from "../../visual-graph/model.js";

export function splitEdges(
  edges: readonly VisualEdge[],
  importSourceIds: ReadonlySet<string>,
): { body: readonly VisualEdge[]; imports: readonly VisualEdge[] } {
  const body: /* mutable */ VisualEdge[] = [];
  const imports: /* mutable */ VisualEdge[] = [];
  for (const e of edges) {
    if (importSourceIds.has(e.from)) {
      imports.push(e);
    } else {
      body.push(e);
    }
  }
  return { body, imports };
}
