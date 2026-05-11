import { VISUAL_ELEMENT_TYPE } from "../../visual-graph/visual-element-type.js";
import type { VisualSubgraph } from "../../visual-graph/visual-subgraph.js";
import { emitNode } from "./emit-node.js";
import { emitSubgraph } from "./emit-subgraph.js";
import type { RenderState } from "./render-state.js";
import { subgraphLabel } from "./subgraph-label.js";
import { nestPaletteIndex } from "./theme/nest-palette-index.js";

export function emitPlainSubgraph(
  state: RenderState,
  sg: VisualSubgraph,
  indent: string,
  depth: number,
): void {
  state.lines.push(
    `${indent}subgraph ${sg.id}["${subgraphLabel(sg, state.nodeMap, state.debug)}"]`,
  );
  recordNestSlot(state, sg.id, depth);
  const childIndent = `${indent}  `;
  state.lines.push(`${childIndent}direction ${sg.direction}`);
  let emittedChildren = 0;
  for (const e of sg.elements) {
    if (
      e.type === VISUAL_ELEMENT_TYPE.Node &&
      !state.wrappedOwnerIds.has(e.id)
    ) {
      emitNode(state, e, childIndent);
      emittedChildren++;
    }
  }
  for (const e of sg.elements) {
    if (e.type === VISUAL_ELEMENT_TYPE.Subgraph) {
      emitSubgraph(state, e, childIndent, depth + 1);
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

function recordNestSlot(
  state: RenderState,
  subgraphId: string,
  depth: number,
): void {
  const paletteLength = state.theme.nestPalette.length;
  if (paletteLength === 0) {
    return;
  }
  const slot = nestPaletteIndex(depth, paletteLength);
  const existing = state.nestClassMap.get(slot);
  if (existing === undefined) {
    state.nestClassMap.set(slot, [subgraphId]);
    return;
  }
  existing.push(subgraphId);
}
