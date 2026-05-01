import { DIRECTION } from "../../../direction.js";
import { SUBGRAPH_KIND } from "../../../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../../visual-element-type.js";
import type { VisualSubgraph } from "../../../visual-graph/model.js";

export function makeSubgraph(
  overrides: Partial<VisualSubgraph> = {},
): VisualSubgraph {
  return {
    type: VISUAL_ELEMENT_TYPE.Subgraph,
    id: "s_x",
    kind: SUBGRAPH_KIND.Function,
    line: 1,
    direction: DIRECTION.RL,
    elements: [],
    ...overrides,
  };
}
