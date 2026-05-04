import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";

export function isSyntheticNode(n: VisualNode): boolean {
  return (
    n.kind === NODE_KIND.ModuleSink ||
    n.kind === NODE_KIND.ModuleSource ||
    n.kind === NODE_KIND.ImportIntermediate ||
    n.kind === NODE_KIND.ExpressionStatement
  );
}
