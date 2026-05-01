import type { VisualGraph } from "../../visual-graph/model.js";
import { emitNode } from "./emit-node.js";
import { isSyntheticNode } from "./is-synthetic-node.js";
import type { RenderState } from "./render-state.js";

export function renderTopLevelNodes(
  state: RenderState,
  graph: VisualGraph,
): void {
  for (const e of graph.elements) {
    if (
      e.type === "node" &&
      !isSyntheticNode(e) &&
      !state.wrappedOwnerIds.has(e.id)
    ) {
      emitNode(state, e, "  ");
    }
  }
}
