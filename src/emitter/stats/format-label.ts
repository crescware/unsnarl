import type { VisualNode } from "../../visual-graph/model.js";

export function formatLabel(path: string, n: VisualNode): string {
  const prefix = n.unused === true ? "unused " : "";
  return `${path}:${n.line} ${prefix}${n.name}`;
}
