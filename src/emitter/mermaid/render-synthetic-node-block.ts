import type { VisualGraph } from "../../visual-graph/model.js";
import { emitNode } from "./emit-node.js";
import { isSyntheticNode } from "./is-synthetic-node.js";
import type { RenderState } from "./render-state.js";

export function renderSyntheticNodeBlock(
  state: RenderState,
  graph: VisualGraph,
): void {
  for (const e of graph.elements) {
    if (e.type === "node" && isSyntheticNode(e)) {
      emitNode(state, e, "  ");
    }
  }
}
