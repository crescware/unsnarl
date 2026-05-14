import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";
import { nodeLabel } from "./node-label.js";

export function nodeSyntax(n: VisualNode, debug: boolean): string {
  const label = nodeLabel(n, debug);
  switch (n.kind) {
    case NODE_KIND.WriteReference:
      return `(["${label}"])`;
    case NODE_KIND.SyntheticModuleSink:
      return `((${label}))`;
    case NODE_KIND.SyntheticBeyondDepth:
      // Circle shape mirrors the pruning boundary stub; both stand in for
      // "more graph keeps going past this rendered boundary".
      return `((${label}))`;
    case NODE_KIND.SyntheticIfStatementTest:
    case NODE_KIND.SyntheticSwitchStatementDiscriminant:
      return `{"${label}"}`;
    default:
      return `["${label}"]`;
  }
}
