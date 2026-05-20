import { SUBGRAPH_KIND } from "../../visual-graph/subgraph-kind.js";
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
    case SUBGRAPH_KIND.Function: {
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

    case SUBGRAPH_KIND.Class:
      if (sg.className === null) {
        return `class (anonymous)<br/>${range}`;
      }
      return `class ${escape(sg.className)}<br/>${range}`;

    case SUBGRAPH_KIND.Switch:
      return `switch ${range}`;

    case SUBGRAPH_KIND.Case:
      if (sg.caseTest === null) {
        return `default ${range}`;
      }
      return `case ${escape(sg.caseTest)} ${range}`;

    case SUBGRAPH_KIND.If:
      return `if ${range}`;

    case SUBGRAPH_KIND.Else:
      return `else ${range}`;

    case SUBGRAPH_KIND.IfElseContainer:
      return `${sg.hasElse ? "if-else" : "if"} ${range}`;

    case SUBGRAPH_KIND.Try:
      return `try ${range}`;

    case SUBGRAPH_KIND.Catch:
      return `catch ${range}`;

    case SUBGRAPH_KIND.Finally:
      return `finally ${range}`;

    case SUBGRAPH_KIND.For:
      return `for ${range}`;

    case SUBGRAPH_KIND.While:
      return `while ${range}`;

    case SUBGRAPH_KIND.DoWhile:
      return `do-while ${range}`;

    case SUBGRAPH_KIND.Return:
      return `return ${range}`;

    case SUBGRAPH_KIND.Throw:
      return `throw ${range}`;

    case SUBGRAPH_KIND.Block:
      return `block ${range}`;
  }
}
