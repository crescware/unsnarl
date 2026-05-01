import { VISUAL_ELEMENT_TYPE } from "../../constants.js";
import type { VisualGraph } from "../../visual-graph/model.js";
import { emitNode } from "./emit-node.js";
import { isSyntheticNode } from "./is-synthetic-node.js";
import type { RenderState } from "./render-state.js";

export function renderSyntheticNodeBlock(
  state: RenderState,
  graph: VisualGraph,
): void {
  for (const e of graph.elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Node && isSyntheticNode(e)) {
      emitNode(state, e, "  ");
    }
  }
}
