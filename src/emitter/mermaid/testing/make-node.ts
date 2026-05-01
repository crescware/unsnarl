import type { VisualNode } from "../../../visual-graph/model.js";

export function makeNode(overrides: Partial<VisualNode> = {}): VisualNode {
  return {
    type: "node",
    id: "n_v",
    kind: "Variable",
    name: "x",
    line: 1,
    isJsxElement: false,
    ...overrides,
  };
}
