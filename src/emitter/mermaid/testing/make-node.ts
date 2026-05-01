import type { VisualNode } from "../../../visual-graph/model.js";
import { NODE_KIND } from "../../../visual-graph/node-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../../../visual-graph/visual-element-type.js";

export function makeNode(overrides: Partial<VisualNode> = {}): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    id: "n_v",
    kind: NODE_KIND.Variable,
    name: "x",
    line: 1,
    isJsxElement: false,
    ...overrides,
  };
}
