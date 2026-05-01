import { NODE_KIND } from "../../constants.js";
import type { VisualNode } from "../../visual-graph/model.js";
import { nodeLabel } from "./node-label.js";

export function nodeSyntax(n: VisualNode): string {
  const label = nodeLabel(n);
  switch (n.kind) {
    case NODE_KIND.WriteOp:
      return `(["${label}"])`;
    case NODE_KIND.ModuleSink:
      return `((${label}))`;
    default:
      return `["${label}"]`;
  }
}
