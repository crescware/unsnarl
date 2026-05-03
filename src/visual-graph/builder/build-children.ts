import type { SerializedScope } from "../../ir/model.js";
import { DIRECTION } from "../direction.js";
import type { VisualElement, VisualNode, VisualSubgraph } from "../model.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import { branchContainerKey } from "./branch-container-key.js";
import { buildScope } from "./build-scope.js";
import type { BuildState } from "./build-state.js";
import type { BuilderContext } from "./context.js";
import { ifContainerSubgraphId } from "./if-container-subgraph-id.js";
import { ifTestNodeId } from "./if-test-node-id.js";
import { lineForOffset } from "./line-for-offset.js";

type Container = Readonly<{
  elements: /* mutable */ VisualElement[];
}>;

function makeIfTestAnchor(
  parentScopeId: string,
  offset: number,
  raw: string,
): VisualNode {
  return {
    type: VISUAL_ELEMENT_TYPE.Node,
    id: ifTestNodeId(parentScopeId, offset),
    kind: NODE_KIND.IfTest,
    name: "if-test",
    line: lineForOffset(raw, offset),
    endLine: null,
    isJsxElement: false,
    unused: false,
  };
}

function pushIfTestAnchor(
  parentScopeId: string,
  offset: number,
  container: Container,
  state: BuildState,
  raw: string,
): void {
  if (state.ifTestAnchorByOffset.has(offset)) {
    return;
  }
  const node = makeIfTestAnchor(parentScopeId, offset, raw);
  container.elements.push(node);
  state.ifTestAnchorByOffset.set(offset, node.id);
}

export function buildChildren(
  parentScope: SerializedScope,
  container: Container,
  ctx: BuilderContext,
  state: BuildState,
): void {
  const children: /* mutable */ SerializedScope[] = [];
  for (const id of parentScope.childScopes) {
    const c = ctx.scopeMap.get(id);
    if (c) {
      children.push(c);
    }
  }
  let i = 0;
  while (i < children.length) {
    const child = children[i];
    if (!child) {
      i++;
      continue;
    }
    const ckey = branchContainerKey(child);
    if (ckey === null || !ckey.startsWith("if:")) {
      buildScope(child, container, ctx, state);
      i++;
      continue;
    }
    const group: /* mutable */ SerializedScope[] = [child];
    let j = i + 1;
    while (j < children.length) {
      const next = children[j];
      if (!next || branchContainerKey(next) !== ckey) {
        break;
      }
      group.push(next);
      j++;
    }
    if (group.length < 2) {
      // Lone `if` (no `else`). Anchor sits at the parent-scope level
      // alongside the consequent subgraph because there is no merge
      // container to host it.
      const lone = group[0];
      const loneOffset = lone?.blockContext?.parentSpanOffset;
      const loneParent = lone?.upper ?? parentScope.id;
      if (lone && loneOffset !== undefined) {
        pushIfTestAnchor(loneParent, loneOffset, container, state, ctx.ir.raw);
      }
      for (const g of group) {
        buildScope(g, container, ctx, state);
      }
      i = j;
      continue;
    }
    const offset = child.blockContext?.parentSpanOffset ?? 0;
    const containerId = ifContainerSubgraphId(child.upper ?? "", offset);
    const hasElse = group.some((g) => g.blockContext?.key === "alternate");
    const containerSubgraph = {
      type: VISUAL_ELEMENT_TYPE.Subgraph,
      id: containerId,
      kind: SUBGRAPH_KIND.IfElseContainer,
      line: lineForOffset(ctx.ir.raw, offset),
      endLine: null as number | null,
      direction: DIRECTION.RL,
      hasElse,
      elements: [] as VisualElement[],
    } satisfies VisualSubgraph;
    container.elements.push(containerSubgraph);
    // Each `IfStatement` in the chain (or the single if/else) gets its
    // own test anchor. Distinct `parentSpanOffset` values within the
    // group correspond to distinct IfStatement nodes; emit one anchor
    // per offset, ahead of the branch subgraphs so it renders first.
    const seenOffsets = new Set<number>();
    for (const g of group) {
      const off = g.blockContext?.parentSpanOffset;
      if (off === undefined || seenOffsets.has(off)) {
        continue;
      }
      seenOffsets.add(off);
      pushIfTestAnchor(
        g.upper ?? "",
        off,
        containerSubgraph,
        state,
        ctx.ir.raw,
      );
    }
    for (const g of group) {
      buildScope(g, containerSubgraph, ctx, state);
    }
    let containerEndLine = containerSubgraph.line;
    for (const elem of containerSubgraph.elements) {
      if (elem.type === VISUAL_ELEMENT_TYPE.Subgraph && elem.endLine !== null) {
        containerEndLine = Math.max(containerEndLine, elem.endLine);
      }
    }
    if (containerEndLine !== containerSubgraph.line) {
      containerSubgraph.endLine = containerEndLine;
    }
    i = j;
  }
}
