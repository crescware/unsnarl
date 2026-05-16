import type { VisualNode } from "../../visual-graph/visual-node.js";
import type { VisualSubgraph } from "../../visual-graph/visual-subgraph.js";
import { escape } from "./escape.js";
import { lineRangeLabel } from "./line-range-label.js";

export function subgraphLabel(
  sg: VisualSubgraph,
  nodeMap: ReadonlyMap<string, VisualNode>,
  debug: boolean,
): string {
  const base = baseLabel(sg, nodeMap);
  return debug ? `${base}<br/>${sg.kind}` : base;
}

function baseLabel(
  sg: VisualSubgraph,
  nodeMap: ReadonlyMap<string, VisualNode>,
): string {
  const range = lineRangeLabel(sg);
  switch (sg.kind) {
    case "function": {
      // Prefer the name baked onto the subgraph at build time; the owner
      // node may be absent after pruning even when the subgraph survives.
      // ownerName is empty when the owner variable was missing at build
      // time -- fall back to the live nodeMap entry in that case.
      if (sg.ownerNodeId === null) {
        return `(anonymous)<br/>${range}`;
      }
      const ownerNode = nodeMap.get(sg.ownerNodeId);
      const name = sg.ownerName !== "" ? sg.ownerName : (ownerNode?.name ?? "");
      return `${escape(name)}()<br/>${range}`;
    }
    case "switch":
      return `switch ${range}`;
    case "case":
      if (sg.caseTest === null) {
        return `default ${range}`;
      }
      return `case ${escape(sg.caseTest)} ${range}`;
    case "if":
      return `if ${range}`;
    case "else":
      return `else ${range}`;
    case "if-else-container":
      return `${sg.hasElse ? "if-else" : "if"} ${range}`;
    case "try":
      return `try ${range}`;
    case "catch":
      return `catch ${range}`;
    case "finally":
      return `finally ${range}`;
    case "for":
      return `for ${range}`;
    case "while":
      return `while ${range}`;
    case "do-while":
      return `do-while ${range}`;
    case "return":
      return `return ${range}`;
    case "throw":
      return `throw ${range}`;
    case "block":
      return `block ${range}`;
  }
}
