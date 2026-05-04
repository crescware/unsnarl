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
import { retUseNodeId } from "./ret-use-node-id.js";
import { returnSubgraphId } from "./return-subgraph-id.js";

export function ensureReturnUseNode(
  enclosingFnVarId: string,
  ref: SerializedReference,
  ctx: BuilderContext,
  state: BuildState,
): string | null {
  const host = findHostSubgraph(ref, enclosingFnVarId, ctx.scopeMap, state);
  if (!host) {
    return null;
  }
  const containerKey = ref.returnContainer
    ? `${ref.returnContainer.startSpan.offset}-${ref.returnContainer.endSpan.offset}`
    : "implicit";
  let perFn = state.returnSubgraphsByFn.get(enclosingFnVarId);
  if (!perFn) {
    perFn = new Map();
    state.returnSubgraphsByFn.set(enclosingFnVarId, perFn);
  }
  const existing = perFn.get(containerKey) ?? null;
  let sg: VisualSubgraph;
  if (existing === null) {
    const startLine = ref.returnContainer?.startSpan.line ?? host.line;
    const rawEndLine = ref.returnContainer?.endSpan.line ?? null;
    const endLine =
      rawEndLine !== null && rawEndLine !== startLine ? rawEndLine : null;
    sg = {
      type: VISUAL_ELEMENT_TYPE.Subgraph,
      id: returnSubgraphId(enclosingFnVarId, containerKey),
      kind: SUBGRAPH_KIND.Return,
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
  const id = retUseNodeId(ref.id);
  if (!state.returnUseAdded.has(ref.id)) {
    state.returnUseAdded.add(ref.id);
    const v = ref.resolved ? (ctx.variableMap.get(ref.resolved) ?? null) : null;
    const name = v?.name ?? ref.identifier.name ?? "";
    const startLine = ref.identifier.span.line;
    const jsxEnd = ref.jsxElement?.endSpan.line ?? null;
    const endLine = jsxEnd !== null && jsxEnd !== startLine ? jsxEnd : null;
    const node = {
      type: VISUAL_ELEMENT_TYPE.Node,
      id,
      kind: NODE_KIND.ReturnUse,
      name,
      line: startLine,
      endLine,
      isJsxElement: ref.jsxElement !== null,
      unused: false,
    } satisfies VisualNode;
    sg.elements.push(node);
  }
  return id;
}
