import type { VisualSubgraph } from "../../visual-graph/visual-subgraph.js";

export function lineRangeLabel(sg: VisualSubgraph): string {
  const end = sg.endLine;
  if (end !== null && end !== sg.line) {
    return `L${sg.line}-${end}`;
  }
  return `L${sg.line}`;
}
