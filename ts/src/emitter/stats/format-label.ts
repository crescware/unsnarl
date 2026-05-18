import type { VisualNode } from "../../visual-graph/visual-node.js";

export function formatLabel(path: string, n: VisualNode): string {
  const prefix = n.unused ? "unused " : "";
  return `${path}:${n.line} ${prefix}${n.name}`;
}
