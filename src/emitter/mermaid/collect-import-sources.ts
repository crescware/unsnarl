import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";

export function collectImportSources(
  nodeMap: ReadonlyMap<string, VisualNode>,
): Set<string> {
  const ids = new Set<string>();
  for (const n of nodeMap.values()) {
    if (
      n.kind === NODE_KIND.ModuleSource ||
      n.kind === NODE_KIND.ImportIntermediate
    ) {
      ids.add(n.id);
    }
  }
  return ids;
}
