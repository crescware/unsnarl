import { NODE_KIND } from "../../constants.js";
import type { VisualNode } from "../../visual-graph/model.js";

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
