import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";

export function isSyntheticNode(n: VisualNode): boolean {
  return (
    n.kind === NODE_KIND.LegacyModuleSink ||
    n.kind === NODE_KIND.LegacyModuleSource ||
    n.kind === NODE_KIND.LegacyImportIntermediate ||
    n.kind === NODE_KIND.LegacyExpressionStatement
  );
}
