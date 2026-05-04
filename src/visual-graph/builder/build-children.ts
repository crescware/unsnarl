import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { DIRECTION } from "../direction.js";
import { NODE_KIND } from "../node-kind.js";
import { SUBGRAPH_KIND } from "../subgraph-kind.js";
import { VISUAL_ELEMENT_TYPE } from "../visual-element-type.js";
import type { VisualElement } from "../visual-element.js";
import type { VisualNode } from "../visual-node.js";
import type { VisualSubgraph } from "../visual-subgraph.js";
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

// Place an IfStatement's test anchor inside the consequent it gates.
// `else` (alternate) is the fallback path and carries no test of its
// own. If the consequent did not materialize as a subgraph, fall back
// to the surrounding container so the anchor is still emitted.
function attachTestAnchorToConsequent(
  consequent: SerializedScope,
  offset: number,
  fallbackContainer: Container,
  state: BuildState,
  raw: string,
): void {
  if (state.ifTestAnchorByOffset.has(offset)) {
    return;
  }
  const bodySg = state.subgraphByScope.get(consequent.id);
  if (bodySg) {
    const node = makeIfTestAnchor(consequent.upper ?? "", offset, raw);
    bodySg.elements.unshift(node);
    state.ifTestAnchorByOffset.set(offset, node.id);
    return;
  }
  pushIfTestAnchor(
    consequent.upper ?? "",
    offset,
    fallbackContainer,
    state,
    raw,
  );
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
      // Lone `if` (no `else`). The consequent subgraph itself stands in
      // for the IfStatement; we skip the IfElseContainer wrapping that
      // exists only to group sibling branches. The test anchor lives
      // inside the consequent it gates (same rule as if-else below).
      const lone = group[0];
      const loneOffset = lone?.blockContext?.parentSpanOffset;
      if (lone) {
        buildScope(lone, container, ctx, state);
        if (loneOffset !== undefined) {
          attachTestAnchorToConsequent(
            lone,
            loneOffset,
            container,
            state,
            ctx.ir.raw,
          );
        }
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
    // Build branch subgraphs first so each consequent has somewhere to
    // host its own test anchor.
    for (const g of group) {
      buildScope(g, containerSubgraph, ctx, state);
    }
    // Each IfStatement's test anchor lives inside the consequent it
    // gates. Distinct `parentSpanOffset` values within the group
    // correspond to distinct IfStatement nodes; the matching consequent
    // (key === "consequent") hosts that test. The `else` (alternate)
    // branch carries no test of its own.
    const seenOffsets = new Set<number>();
    for (const g of group) {
      const off = g.blockContext?.parentSpanOffset;
      if (
        off === undefined ||
        seenOffsets.has(off) ||
        g.blockContext?.key !== "consequent"
      ) {
        continue;
      }
      seenOffsets.add(off);
      attachTestAnchorToConsequent(
        g,
        off,
        containerSubgraph,
        state,
        ctx.ir.raw,
      );
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
