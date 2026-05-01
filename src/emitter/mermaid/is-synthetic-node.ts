import { NODE_KIND } from "../../constants.js";
import type { VisualNode } from "../../visual-graph/model.js";

export function isSyntheticNode(n: VisualNode): boolean {
  return (
    n.kind === NODE_KIND.ModuleSink ||
    n.kind === NODE_KIND.ModuleSource ||
    n.kind === NODE_KIND.ImportIntermediate
  );
}
