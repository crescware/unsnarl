import type { VisualEdge } from "../../visual-graph/visual-edge.js";

export function pushEdgeLines(
  edges: readonly VisualEdge[],
  lines: /* mutable */ string[],
): void {
  for (const e of edges) {
    lines.push(`  ${e.from} -->|${e.label}| ${e.to}`);
  }
}
