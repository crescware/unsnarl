import type { VisualNode } from "../../visual-graph/model.js";

export function isSyntheticNode(n: VisualNode): boolean {
  return (
    n.kind === "ModuleSink" ||
    n.kind === "ModuleSource" ||
    n.kind === "ImportIntermediate"
  );
}
