import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";
import { nodeLabel } from "./node-label.js";

export function nodeSyntax(n: VisualNode, debug: boolean): string {
  const label = nodeLabel(n, debug);
  switch (n.kind) {
    case NODE_KIND.LegacyWriteOp:
      return `(["${label}"])`;
    case NODE_KIND.LegacyModuleSink:
      return `((${label}))`;
    case NODE_KIND.LegacyBeyondDepth:
      // Circle shape mirrors the pruning boundary stub; both stand in for
      // "more graph keeps going past this rendered boundary".
      return `((${label}))`;
    case NODE_KIND.LegacyIfTest:
    case NODE_KIND.LegacySwitchDiscriminant:
      return `{"${label}"}`;
    default:
      return `["${label}"]`;
  }
}
