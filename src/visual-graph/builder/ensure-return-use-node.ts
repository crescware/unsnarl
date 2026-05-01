import type { SerializedReference } from "../../ir/model.js";
import type { VisualNode, VisualSubgraph } from "../model.js";
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
  let sg = perFn.get(containerKey);
  if (!sg) {
    const startLine = ref.returnContainer?.startSpan.line ?? host.line;
    const endLine = ref.returnContainer?.endSpan.line;
    sg = {
      type: "subgraph",
      id: returnSubgraphId(enclosingFnVarId, containerKey),
      kind: "return",
      line: startLine,
      direction: "RL",
      elements: [],
    } satisfies VisualSubgraph;
    if (endLine !== undefined && endLine !== startLine) {
      sg.endLine = endLine;
    }
    host.elements.push(sg);
    perFn.set(containerKey, sg);
  }
  const id = retUseNodeId(ref.id);
  if (!state.returnUseAdded.has(ref.id)) {
    state.returnUseAdded.add(ref.id);
    const v = ref.resolved ? ctx.variableMap.get(ref.resolved) : undefined;
    const name = v?.name ?? ref.identifier.name ?? "";
    const startLine = ref.identifier.span.line;
    const node = {
      type: "node",
      id,
      kind: "ReturnUse",
      name,
      line: startLine,
      isJsxElement: ref.jsxElement !== null,
    } satisfies VisualNode as VisualNode;
    const jsxEnd = ref.jsxElement?.endSpan.line;
    if (jsxEnd !== undefined && jsxEnd !== startLine) {
      node.endLine = jsxEnd;
    }
    sg.elements.push(node);
  }
  return id;
}
