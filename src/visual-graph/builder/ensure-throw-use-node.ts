import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import { DIRECTION } from "../direction.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualNode } from "../visual-node.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { findHostSubgraph } from "./find-host-subgraph.js";
import { throwSubgraphId } from "./throw-subgraph-id.js";
import { throwUseNodeId } from "./throw-use-node-id.js";

export function ensureThrowUseNode(
  enclosingFnVarId: string,
  ref: SerializedReference,
  ctx: BuilderContext,
  state: BuildState,
): string | null {
  if (ref.throwContainer === null) {
    return null;
  }
  const host = findHostSubgraph(ref, enclosingFnVarId, ctx.scopeMap, state);
  if (!host) {
    return null;
  }
  const throwContainer = ref.throwContainer;
  const containerKey = `${throwContainer.startSpan.offset}-${throwContainer.endSpan.offset}`;
  let perFn = state.throwSubgraphsByFn.get(enclosingFnVarId);
  if (!perFn) {
    perFn = new Map();
    state.throwSubgraphsByFn.set(enclosingFnVarId, perFn);
  }
  const existing = perFn.get(containerKey) ?? null;
  let sg: VisualSubgraph;
  if (existing === null) {
    const startLine = throwContainer.startSpan.line;
    const rawEndLine = throwContainer.endSpan.line;
    const endLine = rawEndLine !== startLine ? rawEndLine : null;
    sg = {
      type: VISUAL_ELEMENT_TYPE.Subgraph,
      id: throwSubgraphId(enclosingFnVarId, containerKey),
      kind: SUBGRAPH_KIND.Throw,
      line: startLine,
      endLine,
      direction: DIRECTION.RL,
      elements: [],
    } satisfies VisualSubgraph;
    host.elements.push(sg);
    perFn.set(containerKey, sg);
  } else {
    sg = existing;
  }
  const id = throwUseNodeId(ref.id);
  if (!state.throwUseAdded.has(ref.id)) {
    state.throwUseAdded.add(ref.id);
    const v = ref.resolved ? (ctx.variableMap.get(ref.resolved) ?? null) : null;
    const name = v?.name ?? ref.identifier.name ?? "";
    const startLine = ref.identifier.span.line;
    const jsxEnd = ref.jsxElement?.endSpan.line ?? null;
    const endLine = jsxEnd !== null && jsxEnd !== startLine ? jsxEnd : null;
    const node = {
      type: VISUAL_ELEMENT_TYPE.Node,
      id,
      kind: NODE_KIND.ThrowArgumentReference,
      name,
      line: startLine,
      endLine,
      isJsxElement: ref.jsxElement !== null,
      unused: false,
    } satisfies VisualNode;
    sg.elements.push(node);
    state.nodeIdOriginScope?.set(id, ref.from);
  }
  return id;
}
