import type { VisualEdge } from "../../visual-graph/model.js";

export function splitEdges(
  edges: ReadonlyArray<VisualEdge>,
  importSourceIds: ReadonlySet<string>,
): { body: VisualEdge[]; imports: VisualEdge[] } {
  const body: VisualEdge[] = [];
  const imports: VisualEdge[] = [];
  for (const e of edges) {
    if (importSourceIds.has(e.from)) {
      imports.push(e);
    } else {
      body.push(e);
    }
  }
  return { body, imports };
}
