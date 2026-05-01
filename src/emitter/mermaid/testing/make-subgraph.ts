import type { VisualSubgraph } from "../../../visual-graph/model.js";

export function makeSubgraph(
  overrides: Partial<VisualSubgraph> = {},
): VisualSubgraph {
  return {
    type: "subgraph",
    id: "s_x",
    kind: "function",
    line: 1,
    direction: "RL",
    elements: [],
    ...overrides,
  };
}
