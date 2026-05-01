import type { VisualEdge } from "../../visual-graph/model.js";

export function pushEdgeLines(
  edges: readonly VisualEdge[],
  lines: string[],
): void {
  for (const e of edges) {
    lines.push(`  ${e.from} -->|${e.label}| ${e.to}`);
  }
}
