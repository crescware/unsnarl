import type { VisualSubgraph } from "../../visual-graph/model.js";
import { emitNode } from "./emit-node.js";
import { emitSubgraph } from "./emit-subgraph.js";
import type { RenderState } from "./render-state.js";
import { subgraphLabel } from "./subgraph-label.js";

export function emitPlainSubgraph(
  state: RenderState,
  sg: VisualSubgraph,
  indent: string,
): void {
  state.lines.push(
    `${indent}subgraph ${sg.id}["${subgraphLabel(sg, state.nodeMap)}"]`,
  );
  const childIndent = `${indent}  `;
  state.lines.push(`${childIndent}direction ${sg.direction}`);
  let emittedChildren = 0;
  for (const e of sg.elements) {
    if (e.type === "node" && !state.wrappedOwnerIds.has(e.id)) {
      emitNode(state, e, childIndent);
      emittedChildren++;
    }
  }
  for (const e of sg.elements) {
    if (e.type === "subgraph") {
      emitSubgraph(state, e, childIndent);
      emittedChildren++;
    }
  }
  if (emittedChildren === 0) {
    const patch = state.strategy.emptySubgraphPlaceholder({
      subgraphId: sg.id,
      indent: childIndent,
      referencedByEdge: state.edgeEndpointIds.has(sg.id),
    });
    if (patch !== null) {
      state.lines.push(patch.line);
      state.placeholderIds.push(patch.placeholderId);
    }
  }
  state.lines.push(`${indent}end`);
}
