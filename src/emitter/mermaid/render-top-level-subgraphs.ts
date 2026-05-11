import { VISUAL_ELEMENT_TYPE } from "../../visual-graph/visual-element-type.js";
import type { VisualGraph } from "../../visual-graph/visual-graph.js";
import { emitSubgraph } from "./emit-subgraph.js";
import type { RenderState } from "./render-state.js";

export function renderTopLevelSubgraphs(
  state: RenderState,
  graph: VisualGraph,
): void {
  for (const e of graph.elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Subgraph) {
      // Top-level subgraphs sit at nesting depth 1 (palette slot 0). The
      // depth is 1-based throughout so it matches the user-facing `nestL<n>`
      // class names.
      emitSubgraph(state, e, "  ", 1);
    }
  }
}
