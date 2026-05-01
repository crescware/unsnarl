import { VISUAL_ELEMENT_TYPE } from "../../visual-element-type.js";
import type { VisualGraph } from "../../visual-graph/model.js";
import { emitSubgraph } from "./emit-subgraph.js";
import type { RenderState } from "./render-state.js";

export function renderTopLevelSubgraphs(
  state: RenderState,
  graph: VisualGraph,
): void {
  for (const e of graph.elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Subgraph) {
      emitSubgraph(state, e, "  ");
    }
  }
}
