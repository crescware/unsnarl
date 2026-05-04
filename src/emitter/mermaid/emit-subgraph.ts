import { SUBGRAPH_KIND } from "../../visual-graph/subgraph-kind.js";
import type { VisualSubgraph } from "../../visual-graph/visual-subgraph.js";
import { emitNode } from "./emit-node.js";
import { emitPlainSubgraph } from "./emit-plain-subgraph.js";
import type { RenderState } from "./render-state.js";

export function emitSubgraph(
  state: RenderState,
  sg: VisualSubgraph,
  indent: string,
): void {
  if (sg.kind === SUBGRAPH_KIND.Function && sg.ownerNodeId !== null) {
    const ownerNode = state.nodeMap.get(sg.ownerNodeId) ?? null;
    if (ownerNode !== null) {
      // Wrap the FunctionName node and the function body subgraph as
      // SIBLINGS inside a single wrapper subgraph. The FunctionName node
      // belongs to the parent scope (it names the function from the
      // outside), so it must NOT live inside the body subgraph -- that
      // would imply "f references itself from within its own body".
      // The wrapper exists purely to keep these two siblings adjacent in
      // the rendered diagram.
      const wrapId = `wrap_${sg.id}`;
      state.wrapperIds.push(wrapId);
      state.lines.push(`${indent}subgraph ${wrapId}[" "]`);
      const wrapIndent = `${indent}  `;
      state.lines.push(`${wrapIndent}direction TB`);
      emitNode(state, ownerNode, wrapIndent);
      emitPlainSubgraph(state, sg, wrapIndent);
      state.lines.push(`${indent}end`);
      return;
    }
  }
  emitPlainSubgraph(state, sg, indent);
}
