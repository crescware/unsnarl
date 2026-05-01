import { DIRECTION, VISUAL_ELEMENT_TYPE } from "../../../constants.js";
import type { VisualSubgraph } from "../../../visual-graph/model.js";

export function makeSubgraph(
  overrides: Partial<VisualSubgraph> = {},
): VisualSubgraph {
  return {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id: "s_x",
    kind: "function",
    line: 1,
    direction: DIRECTION.RL,
    elements: [],
    ...overrides,
  };
}
