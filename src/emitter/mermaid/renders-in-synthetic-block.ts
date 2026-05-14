import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";

export function rendersInSyntheticBlock(n: VisualNode): boolean {
  return (
    n.kind === NODE_KIND.SyntheticModuleSink ||
    n.kind === NODE_KIND.SyntheticModuleSource ||
    n.kind === NODE_KIND.SyntheticImportIntermediate ||
    n.kind === NODE_KIND.SyntheticExpressionStatement
  );
}
