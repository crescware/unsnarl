import type { VisualNode } from "../../visual-graph/model.js";
import { nodeLabel } from "./node-label.js";

export function nodeSyntax(n: VisualNode): string {
  const label = nodeLabel(n);
  switch (n.kind) {
    case "WriteOp":
      return `(["${label}"])`;
    case "ModuleSink":
      return `((${label}))`;
    default:
      return `["${label}"]`;
  }
}
