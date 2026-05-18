import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualEdge } from "../../visual-graph/visual-edge.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";

export function pushEdgeLines(
  edges: readonly VisualEdge[],
  lines: /* mutable */ string[],
  nodeMap?: ReadonlyMap<string, VisualNode>,
): void {
  for (const e of edges) {
    const arrow = touchesBeyondDepth(e, nodeMap) ? "-.->" : "-->";
    lines.push(`  ${e.from} ${arrow}|${e.label}| ${e.to}`);
  }
}

// Edges that point at (or away from) a BeyondDepth `((...))` stub render
// with a dashed arrow so the boundary into the hidden subtree is visually
// consistent with the pruning boundary edges.
function touchesBeyondDepth(
  e: VisualEdge,
  nodeMap: ReadonlyMap<string, VisualNode> | undefined,
): boolean {
  if (!nodeMap) {
    return false;
  }
  return (
    nodeMap.get(e.from)?.kind === NODE_KIND.SyntheticBeyondDepth ||
    nodeMap.get(e.to)?.kind === NODE_KIND.SyntheticBeyondDepth
  );
}
