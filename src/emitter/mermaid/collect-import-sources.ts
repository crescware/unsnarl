import type { VisualNode } from "../../visual-graph/model.js";

export function collectImportSources(
  nodeMap: ReadonlyMap<string, VisualNode>,
): Set<string> {
  const ids = new Set<string>();
  for (const n of nodeMap.values()) {
    if (n.kind === "ModuleSource" || n.kind === "ImportIntermediate") {
      ids.add(n.id);
    }
  }
  return ids;
}
